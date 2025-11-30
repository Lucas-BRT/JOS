use base64::Engine;
use chrono::Utc;
use domain::auth::*;
use domain::entities::*;
use domain::repositories::{RefreshTokenRepository, UserRepository};
use log::warn;
use rand::Rng;
use rand::RngCore;
use shared::Result;
use shared::error::ApplicationError;
use shared::error::DomainError;
use shared::error::Error;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    password_provider: Arc<dyn PasswordProvider>,
    jwt_provider: Arc<dyn TokenProvider>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    jwt_expiration_duration: Duration,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_provider: Arc<dyn PasswordProvider>,
        jwt_provider: Arc<dyn TokenProvider>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
        jwt_expiration_duration: Duration,
    ) -> Self {
        Self {
            user_repository,
            password_provider,
            jwt_provider,
            refresh_token_repository,
            jwt_expiration_duration,
        }
    }

    async fn issue_refresh_token(&self, user_id: Uuid) -> Result<String> {
        self.refresh_token_repository
            .delete_by_user(user_id)
            .await?;

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        let command = CreateRefreshTokenCommand {
            id: Uuid::now_v7(),
            user_id,
            token: token.clone(),
            expires_at: Utc::now() + self.jwt_expiration_duration,
        };

        self.refresh_token_repository.create(command).await?;
        Ok(token)
    }

    async fn validate_credentials(&self, email: &str, password: &str) -> Result<Option<User>> {
        let user = self.user_repository.find_by_email(email).await?;

        match user {
            Some(user) => {
                let is_valid = self
                    .password_provider
                    .verify_hash(password.to_string(), user.password.clone())
                    .await?;

                if is_valid {
                    Ok(Some(user))
                } else {
                    let time_to_sleep = rand::thread_rng()
                        .gen_range(DEFAULT_MIN_DELAY_MILIS..DEFAULT_MAX_DELAY_MILIS);
                    std::thread::sleep(Duration::from_millis(time_to_sleep));
                    Ok(None)
                }
            }
            None => {
                let time_to_sleep =
                    rand::thread_rng().gen_range(DEFAULT_MIN_DELAY_MILIS..DEFAULT_MAX_DELAY_MILIS);
                std::thread::sleep(Duration::from_millis(time_to_sleep));
                Ok(None)
            }
        }
    }
}

#[async_trait::async_trait]
impl AuthenticationService for AuthService {
    async fn login(&self, command: LoginCommand) -> Result<LoginResponse> {
        let user = self
            .validate_credentials(&command.email, &command.password)
            .await?
            .ok_or_else(|| Error::Application(ApplicationError::InvalidCredentials))?;

        let access_token = self.jwt_provider.generate_token(user.id).await?;
        let refresh_token = self.issue_refresh_token(user.id).await?;
        let expires_in = (Utc::now() + self.jwt_expiration_duration).timestamp_millis();

        Ok(LoginResponse {
            user,
            access_token,
            refresh_token,
            expires_in,
        })
    }

    async fn register(&self, command: RegisterCommand) -> Result<LoginResponse> {
        let hashed_password = self
            .password_provider
            .generate_hash(command.password.clone())
            .await?;

        let create_command = CreateUserCommand {
            id: Uuid::now_v7(),
            username: command.username,
            email: command.email.clone(),
            password: hashed_password,
        };

        self.user_repository.create(create_command).await?;

        let login_command = LoginCommand {
            email: command.email,
            password: command.password,
        };

        self.login(login_command).await
    }

    async fn change_password(&self, user_id: Uuid, command: ChangePasswordCommand) -> Result<()> {
        let update_command = UpdatePasswordCommand {
            user_id,
            current_password: command.current_password,
            new_password: command.new_password,
        };

        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| {
                Error::Domain(DomainError::EntityNotFound {
                    entity_type: "User",
                    entity_id: user_id.to_string(),
                })
            })?;

        if !self
            .password_provider
            .verify_hash(
                update_command.current_password.clone(),
                user.password.clone(),
            )
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        let new_password_hash = self
            .password_provider
            .generate_hash(update_command.new_password.clone())
            .await?;

        let user_update_command = UpdateUserCommand {
            user_id,
            password: Some(new_password_hash),
            ..Default::default()
        };

        self.user_repository.update(user_update_command).await?;

        Ok(())
    }

    async fn refresh_token(&self, command: RefreshTokenCommand) -> Result<RefreshResponse> {
        let existing = self
            .refresh_token_repository
            .find_by_token(&command.token)
            .await?;

        let record = existing.ok_or_else(|| {
            warn!("Invalid refresh token");
            Error::Application(ApplicationError::InvalidCredentials)
        })?;

        if record.expires_at < Utc::now() {
            self.refresh_token_repository
                .delete_by_token(&command.token)
                .await?;
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.refresh_token_repository
            .delete_by_token(&command.token)
            .await?;

        let access_token = self.jwt_provider.generate_token(record.user_id).await?;
        let new_refresh_token = self.issue_refresh_token(record.user_id).await?;

        Ok(RefreshResponse {
            access_token,
            refresh_token: new_refresh_token,
            expires_in: self.jwt_expiration_duration.as_secs(),
        })
    }

    async fn logout(&self, command: LogoutCommand) -> Result<()> {
        self.refresh_token_repository
            .delete_by_user(command.user_id)
            .await?;
        Ok(())
    }

    async fn validate_token(&self, token: &str) -> Result<Claims> {
        self.jwt_provider.decode_token(token).await
    }
}
