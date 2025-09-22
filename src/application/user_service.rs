use crate::Result;
use crate::domain::entities::*;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create(&self, command: &CreateUserCommand) -> Result<User> {
        self.user_repository.create(command).await
    }

    pub async fn get(&self, command: &GetUserCommand) -> Result<Vec<User>> {
        self.user_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        let command = GetUserCommand {
            id: Some(*id),
            ..Default::default()
        };
        let users = self.user_repository.read(&command).await?;
        users.into_iter().next()
            .ok_or_else(|| crate::Error::Domain(crate::domain::error::DomainError::NotFound))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User> {
        let command = GetUserCommand {
            username: Some(username.to_string()),
            ..Default::default()
        };
        let users = self.user_repository.read(&command).await?;
        users.into_iter().next()
            .ok_or_else(|| crate::Error::Domain(crate::domain::error::DomainError::NotFound))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User> {
        let command = GetUserCommand {
            email: Some(email.to_string()),
            ..Default::default()
        };
        let users = self.user_repository.read(&command).await?;
        users.into_iter().next()
            .ok_or_else(|| crate::Error::Domain(crate::domain::error::DomainError::NotFound))
    }

    pub async fn update(&self, command: &UpdateUserCommand) -> Result<User> {
        self.user_repository.update(command).await
    }

    pub async fn delete(&self, command: &DeleteUserCommand) -> Result<User> {
        self.user_repository.delete(command).await
    }
}
