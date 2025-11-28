use crate::entities::*;
use crate::repositories::base::Repository;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository:
    Repository<User, CreateUserCommand, UpdateUserCommand, GetUserCommand, DeleteUserCommand>
    + Send
    + Sync
{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<()>;
    async fn search(&self, query: &str) -> Result<Vec<User>>;
}
