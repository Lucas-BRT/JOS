use domain::auth::PasswordProvider;
use domain::entities::commands::UpdatePasswordCommand;
use domain::repositories::UserRepository;
use shared::error::{ApplicationError, DomainError};
use shared::{Error, Result};
use std::sync::Arc;

#[derive(Clone)]
pub struct PasswordService {
    password_provider: Arc<dyn PasswordProvider>,
    user_repository: Arc<dyn UserRepository>,
}

impl PasswordService {
    pub fn new(
        password_provider: Arc<dyn PasswordProvider>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            password_provider,
            user_repository,
        }
    }

    pub async fn generate_hash(&self, password: String) -> Result<String> {
        self.password_provider.generate_hash(password).await
    }

    pub async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        self.password_provider.verify_hash(password, hash).await
    }

    pub async fn validate_password(&self, password: &str) -> Result<()> {
        self.password_provider.validate_password(password).await
    }

    pub async fn hash_password(&self, password: String) -> Result<String> {
        self.generate_hash(password).await
    }

    pub async fn update_password(&self, command: UpdatePasswordCommand) -> Result<()> {
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
            .verify_hash(command.current_password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        let new_password_hash = self.generate_hash(command.new_password.clone()).await?;

        let update_command = domain::entities::commands::UpdateUserCommand {
            user_id: command.user_id,
            username: None,
            email: None,
            password: Some(new_password_hash),
        };

        self.user_repository.update(update_command).await?;

        Ok(())
    }
}
