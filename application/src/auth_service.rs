use base64::Engine;
use chrono::Duration;
use chrono::Utc;
use domain::auth::*;
use domain::entities::*;
use domain::repositories::{RefreshTokenRepository, UserRepository};
use log::warn;
use rand::RngCore;
use shared::Result;
use shared::error::ApplicationError;
use shared::error::DomainError;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: Arc<dyn UserRepository>,
    pub password_provider: Arc<dyn PasswordProvider>,
    pub jwt_provider: Arc<dyn TokenProvider>,
    pub refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_provider: Arc<dyn PasswordProvider>,
        jwt_provider: Arc<dyn TokenProvider>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    ) -> Self {
        Self {
            user_repository,
            password_provider,
            jwt_provider,
            refresh_token_repository,
        }
    }

    pub async fn issue_refresh_token(&self, user_id: Uuid, expiration: Duration) -> Result<String> {
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
            expires_at: Utc::now() + expiration,
        };

        self.refresh_token_repository.create(command).await?;
        Ok(token)
    }

    pub async fn rotate_refresh_token(
        &self,
        old_token: &str,
        refresh_token_duration: Duration,
    ) -> Result<(String, Uuid)> {
        let existing = self
            .refresh_token_repository
            .find_by_token(old_token)
            .await?;

        let record = match existing {
            Some(r) => r,
            None => {
                warn!("Invalid refresh token");
                return Err(Error::Application(
                    shared::error::ApplicationError::InvalidCredentials,
                ));
            }
        };

        if record.expires_at < Utc::now() {
            // delete expired token
            self.refresh_token_repository
                .delete_by_token(old_token)
                .await?;

            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        // rotate: delete old and issue new for same user
        self.refresh_token_repository
            .delete_by_token(old_token)
            .await?;

        let new_token = self
            .issue_refresh_token(record.user_id, refresh_token_duration)
            .await?;
        Ok((new_token, record.user_id))
    }
}

#[async_trait::async_trait]
impl Authenticator for AuthService {
    async fn authenticate(&self, payload: LoginUserCommand) -> Result<String> {
        let user = match self.user_repository.find_by_email(&payload.email).await? {
            Some(user) => user,
            None => {
                return Err(Error::Application(
                    shared::error::ApplicationError::InvalidCredentials,
                ));
            }
        };

        if !self
            .password_provider
            .verify_hash(payload.password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(
                shared::error::ApplicationError::InvalidCredentials,
            ));
        }

        let jwt_token = self.jwt_provider.generate_token(&user.id).await?;

        Ok(jwt_token)
    }

    async fn register(&self, payload: CreateUserCommand) -> Result<User> {
        let mut payload = payload;

        payload.password = self
            .password_provider
            .generate_hash(payload.password.clone())
            .await?;

        let created_user = self.user_repository.create(payload.clone()).await?;

        Ok(created_user)
    }

    async fn update_password(&self, payload: UpdatePasswordCommand) -> Result<()> {
        let user = self
            .user_repository
            .find_by_id(payload.user_id)
            .await?
            .ok_or_else(|| {
                Error::Domain(DomainError::EntityNotFound {
                    entity_type: "User",
                    entity_id: payload.user_id.to_string(),
                })
            })?;

        if !self
            .password_provider
            .verify_hash(payload.current_password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        let new_password_hash = self
            .password_provider
            .generate_hash(payload.new_password.clone())
            .await?;

        let command = UpdateUserCommand {
            user_id: payload.user_id,
            username: None,
            email: None,
            password: Some(new_password_hash),
        };

        self.user_repository.update(command).await?;

        Ok(())
    }

    async fn logout(&self, user_id: Uuid) -> Result<()> {
        self.refresh_token_repository
            .delete_by_user(user_id)
            .await?;

        Ok(())
    }

    async fn delete_account(&self, command: DeleteAccountCommand) -> Result<()> {
        let user = self
            .user_repository
            .find_by_id(command.user_id)
            .await?
            .ok_or_else(|| {
                Error::Domain(DomainError::EntityNotFound {
                    entity_type: "User",
                    entity_id: command.user_id.to_string(),
                })
            })?;

        if !self
            .password_provider
            .verify_hash(command.password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        self.user_repository.delete_by_id(&command.user_id).await?;

        Ok(())
    }
}
