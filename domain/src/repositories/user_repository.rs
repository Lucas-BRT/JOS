use shared::Result;
use crate::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, command: &mut CreateUserCommand) -> Result<User>;
    async fn read(&self, command: &mut GetUserCommand) -> Result<Vec<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>>;
    async fn update(&self, command: &mut UpdateUserCommand) -> Result<User>;
    async fn delete(&self, command: &mut DeleteUserCommand) -> Result<User>;
    async fn search(&self, query: &str) -> Result<Vec<User>>;
}
