use crate::{
    Error, Result,
    application::error::ApplicationError,
    domain::user::{
        dtos::{CreateUserCommand, LoginUserCommand},
        entity::User,
        user_repository::UserRepository,
    },
    application::services::jwt_service::JwtService,
    utils::password::verify_hash,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: JwtService,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: JwtService) -> Self {
        Self { user_repository, jwt_service }
    }

    pub async fn signup(&self, new_user_data: &CreateUserCommand) -> Result<User> {
        let user = self.user_repository.create(new_user_data).await?;
        Ok(user)
    }

    pub async fn login(&self, login_payload: &LoginUserCommand) -> Result<String> {
        let user = self
            .user_repository
            .find_by_email(&login_payload.email)
            .await?;

        if verify_hash(login_payload.password.clone(), user.password_hash.clone()).await? {
            let jwt_token = self.jwt_service.generate_token(user.id, user.role).await?;
            Ok(jwt_token)
        } else {
            Err(Error::Application(ApplicationError::InvalidCredentials))
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        self.user_repository.find_by_username(username).await
    }

    pub async fn get(&self) -> Result<Vec<User>> {
        self.user_repository.get_all().await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        self.user_repository.find_by_id(id).await
    }
}
