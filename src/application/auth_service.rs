use crate::{
    Error, Result,
    application::error::ApplicationError,
    domain::{
        auth::{Authenticator, TokenProvider},
        password::PasswordProvider,
        user::{
            UpdateUserCommand, UserRepository,
            commands::{CreateUserCommand, LoginUserCommand},
            entity::User,
        },
        utils::update::Update,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthService {
    pub user_repository: Arc<dyn UserRepository>,
    pub password_provider: Arc<dyn PasswordProvider>,
    pub jwt_provider: Arc<dyn TokenProvider>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_provider: Arc<dyn PasswordProvider>,
        jwt_provider: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            user_repository,
            password_provider,
            jwt_provider,
        }
    }
}

#[async_trait::async_trait]
impl Authenticator for AuthService {
    async fn authenticate(&self, payload: &LoginUserCommand) -> Result<String> {
        let user = self.user_repository.find_by_email(&payload.email).await?;
        if !self
            .password_provider
            .verify_hash(payload.password.clone(), user.password_hash.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        let jwt_token = self.jwt_provider.generate_token(user.id, user.role).await?;

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

    async fn update_password(&self, payload: &mut UpdateUserCommand) -> Result<()> {
        match payload.password {
            Update::Keep => return Err(Error::Application(ApplicationError::InvalidCredentials)),
            Update::Change(password) => {
                self.password_provider.validate_password(password).await?;
                Ok(())
            }
        }
    }
}
