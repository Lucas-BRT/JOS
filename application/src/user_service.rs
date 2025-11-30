use domain::auth::PasswordProvider;
use domain::entities::*;
use domain::repositories::UserRepository;
use shared::Result;
use shared::error::Error;
use shared::error::{ApplicationError, DomainError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateProfileCommand {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    password_provider: Arc<dyn PasswordProvider>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_provider: Arc<dyn PasswordProvider>,
    ) -> Self {
        Self {
            user_repository,
            password_provider,
        }
    }

    pub async fn create(&self, command: CreateUserCommand) -> Result<User> {
        self.user_repository.create(command).await
    }

    pub async fn get(&self, command: GetUserCommand) -> Result<Vec<User>> {
        self.user_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<User> {
        let users = self.user_repository.find_by_id(*id).await?;
        users.ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: id.to_string(),
            })
        })
    }

    pub async fn update(&self, command: UpdateUserCommand) -> Result<User> {
        self.user_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteUserCommand) -> Result<User> {
        self.user_repository.delete(command).await
    }

    pub async fn delete_account(&self, command: DeleteAccountCommand) -> Result<()> {
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

        if user.id != command.user_id {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        if !self
            .password_provider
            .verify_hash(command.password.clone(), user.password.clone())
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        let delete_command = DeleteUserCommand {
            id: command.user_id,
        };

        self.user_repository.delete(delete_command).await?;

        Ok(())
    }

    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<ProfileResponse> {
        let user = self.find_by_id(user_id).await?.ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: user_id.to_string(),
            })
        })?;

        Ok(ProfileResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            joined_at: user.created_at,
        })
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        command: UpdateProfileCommand,
    ) -> Result<ProfileResponse> {
        let update_command = UpdateUserCommand {
            user_id,
            username: command.username,
            email: command.email,
            password: None,
        };

        let updated_user = self.update(update_command).await?;

        Ok(ProfileResponse {
            id: updated_user.id,
            username: updated_user.username,
            email: updated_user.email,
            joined_at: updated_user.created_at,
        })
    }
}
