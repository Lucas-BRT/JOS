use shared::Result;
use crate::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, command: &CreateUserCommand) -> Result<User>;
    async fn get(&self, command: &GetUserCommand) -> Result<Vec<User>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<User>;
    async fn find_by_username(&self, username: &str) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
    async fn update(&self, command: &UpdateUserCommand) -> Result<User>;
    async fn delete(&self, command: &DeleteUserCommand) -> Result<User>;
}
