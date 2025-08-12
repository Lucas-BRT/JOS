use crate::{
    application::{error::ApplicationError, services::{jwt_service::JwtService, password_service::PasswordService}}, domain::user::{
        dtos::{CreateUserCommand, LoginUserCommand}, entity::User, role::Role, user_repository::UserRepository
    }, interfaces::http::auth::dtos::SignupDto, Error, Result
};
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn signup(&self, user_data: SignupDto) -> Result<User> {
        let password_hash = self.password_service.generate_hash(user_data.password.clone()).await?;
        let user_data = CreateUserCommand {
            password_hash,
            ..user_data.into()
        };

        let user = self.user_repository.create(user_data).await?;
        Ok(user)
    }

    pub async fn login(&self, login_payload: LoginUserCommand) -> Result<String> {
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
}
