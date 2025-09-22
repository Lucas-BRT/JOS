use crate::Result;
use crate::domain::entities::{CreateUserCommand, DeleteUserCommand, UpdateUserCommand};
use crate::domain::entities::{GetUserCommand, User};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, command: CreateUserCommand) -> Result<User>;
    async fn read(&self, command: GetUserCommand) -> Result<Vec<User>>;
    async fn update(&self, command: UpdateUserCommand) -> Result<User>;
    async fn delete(&self, command: DeleteUserCommand) -> Result<User>;
}
