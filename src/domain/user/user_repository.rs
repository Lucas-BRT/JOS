use super::commands::{CreateUserCommand, UpdateUserCommand};
use super::entity::User;
use crate::Result;
use crate::domain::user::search_commands::UserFilters;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &CreateUserCommand) -> Result<User>;
    async fn get_all(&self, filters: &UserFilters) -> Result<Vec<User>>;
    async fn find_by_username(&self, name: &str) -> Result<User>;
    async fn find_by_id(&self, id: &Uuid) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
    async fn update(&self, data: &UpdateUserCommand) -> Result<()>;
    async fn delete(&self, user_id: &Uuid) -> Result<User>;
}
