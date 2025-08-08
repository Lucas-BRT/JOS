use crate::{
    Error, Result,
    application::error::ApplicationError,
    domain::user::{
        dtos::{CreateUserCommand, LoginUserCommand},
        entity::User,
        user_repository::UserRepository,
        role::Role,
    },
    application::services::{jwt_service::JwtService, password_service::PasswordService},
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: JwtService,
    password_service: PasswordService,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>, 
        jwt_service: JwtService,
        password_service: PasswordService,
    ) -> Self {
        Self { user_repository, jwt_service, password_service }
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

        if self.password_service.verify_hash(login_payload.password.clone(), user.password_hash.clone()).await? {
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

    pub async fn create_user(&self, user_data: &CreateUserCommand) -> Result<User> {
        // Validate password before creating user
        self.password_service.validate_password(&user_data.password)
            .await
            .map_err(|e| {
                tracing::error!("Password validation failed: {}", e.message);
                Error::Application(ApplicationError::InvalidInput(e.message))
            })?;

        let password_hash = self.password_service.generate_hash(user_data.password.clone()).await?;

        let user = User {
            id: Uuid::new_v4(),
            name: user_data.name.clone(),
            email: user_data.email.clone(),
            password_hash,
            role: Role::User, // Default role
            created_at: Utc::now(),
            updated_at: None,
        };

        self.user_repository.create(user_data).await
    }
}
