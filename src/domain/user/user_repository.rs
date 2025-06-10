use super::dtos::{CreateUserCommand, UpdateUserCommand};
use super::entity::User;
use crate::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &CreateUserCommand) -> Result<User>;
    async fn update(&self, user_id: &Uuid, data: &UpdateUserCommand) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<User>>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
}
