use domain::entities::*;
use domain::repositories::UserRepository;
use shared::Result;
use shared::error::DomainError;
use shared::error::Error;
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

    pub async fn create(&self, command: &mut CreateUserCommand) -> Result<User> {
        self.user_repository.create(command).await
    }

    pub async fn get(&self, command: &mut GetUserCommand) -> Result<Vec<User>> {
        self.user_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        let users = self.user_repository.find_by_id(id).await?;
        users.into_iter().next().ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: id.to_string(),
            })
        })
    }

    pub async fn update(&self, command: &mut UpdateUserCommand) -> Result<User> {
        self.user_repository.update(command).await
    }

    pub async fn delete(&self, command: &mut DeleteUserCommand) -> Result<User> {
        self.user_repository.delete(command).await
    }
}
