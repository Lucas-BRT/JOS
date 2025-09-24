use crate::application::error::ApplicationError;
use crate::domain::auth::*;
use crate::domain::entities::*;
use crate::domain::error::*;
use crate::domain::repositories::UserRepository;
use crate::domain::utils::update::Update;
use crate::{Error, Result};
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
    async fn authenticate(&self, payload: &mut LoginUserCommand) -> Result<String> {
        let user = match self.user_repository.find_by_email(&payload.email).await? {
            Some(user) => user,
            None => return Err(Error::Domain(UserDomainError::UserNotFound.into())),
        };

        if !self
            .password_provider
            .verify_hash(payload.password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
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
                return Err(Error::Domain(DomainError::User(
                    UserDomainError::UserNotFound,
                )));
            }
        }
    }
}
