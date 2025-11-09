use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IUserService: Send + Sync {
    async fn create(&self, command: &CreateUserCommand) -> Result<User, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn update(&self, command: &UpdateUserCommand) -> Result<User, Error>;
    async fn delete(&self, command: &mut DeleteUserCommand) -> Result<User, Error>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}
