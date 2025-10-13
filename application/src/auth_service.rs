use base64::Engine;
use chrono::Utc;
use domain::auth::*;
use domain::entities::*;
use domain::repositories::{RefreshTokenRepository, UserRepository};
use log::warn;
use rand::RngCore;
use shared::Result;
use shared::error::ApplicationError;
use shared::error::Error;
use std::sync::Arc;
use uuid::{NoContext, Uuid};

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

    pub async fn issue_refresh_token(&self, user_id: &Uuid) -> Result<String> {
        self.refresh_token_repository
            .delete_by_user(user_id)
            .await?;

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token_str = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        let rt = RefreshToken {
            id: Uuid::new_v7(uuid::Timestamp::now(NoContext)),
            user_id: *user_id,
            token: token_str.clone(),
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
        };

        self.refresh_token_repository.create(&rt).await?;
        Ok(token_str)
    }

    pub async fn rotate_refresh_token(&self, old_token: &str) -> Result<(String, Uuid)> {
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

        let new_token = self.issue_refresh_token(&record.user_id).await?;
        Ok((new_token, record.user_id))
    }
}

#[async_trait::async_trait]
impl Authenticator for AuthService {
    async fn authenticate(&self, payload: &mut LoginUserCommand) -> Result<String> {
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

    async fn register(&self, payload: &mut CreateUserCommand) -> Result<User> {
        payload.password = self
            .password_provider
            .generate_hash(payload.password.clone())
            .await?;

        let created_user = self.user_repository.create(payload).await?;

        Ok(created_user)
    }

    async fn update_password(&self, payload: &mut UpdatePasswordCommand) -> Result<()> {
        let user = self.user_repository.find_by_id(&payload.user_id).await?;

        match user {
            Some(user) => {
                let new_passoword_hash =
                    self.password_provider.generate_hash(user.password).await?;

                let mut command = UpdateUserCommand {
                    password: Update::Change(new_passoword_hash),
                    ..Default::default()
                };

                self.user_repository.update(&mut command).await?;

                Ok(())
            }
            None => {
                return Err(Error::Domain(shared::error::DomainError::EntityNotFound(
                    format!("User not found: {}", payload.user_id),
                )));
            }
        }
    }

    async fn logout(&self, user_id: &Uuid) -> Result<()> {
        Ok(self
            .refresh_token_repository
            .delete_by_user(user_id)
            .await?)
    }
}
