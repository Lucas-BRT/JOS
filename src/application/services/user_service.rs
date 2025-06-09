use crate::{
    Error, Result,
    application::error::ApplicationError,
    domain::user::{
        dtos::{CreateUserCommand, LoginUserCommand},
        entity::User,
        user_repository::UserRepository,
    },
    error::ValidationError,
    utils::{jwt::Claims, password::verify_hash},
};
use chrono::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn signup(&self, new_user_data: &CreateUserCommand) -> Result<User> {
        if new_user_data.password != new_user_data.confirm_password {
            return Err(Error::Validation(ValidationError::PasswordMismatch));
        }

        let user = self.user_repository.create(new_user_data).await?;
        Ok(user)
    }

    pub async fn login(
        &self,
        login_payload: &LoginUserCommand,
        jwt_secret: &str,
        jwt_expiration_duration: Duration,
    ) -> Result<String> {
        let user = self
            .user_repository
            .find_by_email(&login_payload.email)
            .await?;

        if verify_hash(login_payload.password.clone(), user.password_hash.clone()).await? {
            let jwt_token = Claims::create_jwt(
                user.id,
                jwt_secret,
                jwt_expiration_duration,
                user.access_level,
            )?;

            Ok(jwt_token)
        } else {
            Err(Error::Application(ApplicationError::InvalidCredentials))
        }
    }

    pub async fn find_by_username(&self, _username: &str) -> Result<User> {
        todo!()
    }

    pub async fn get(&self) -> Result<Vec<User>> {
        Ok(self.user_repository.get_all().await?)
    }
}
