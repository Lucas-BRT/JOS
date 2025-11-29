use crate::entities::*;
use crate::repositories::base::Repository;
use shared::Result;

#[async_trait::async_trait]
pub trait UserRepository:
    Repository<User, CreateUserCommand, UpdateUserCommand, GetUserCommand, DeleteUserCommand>
    + Send
    + Sync
{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
}
