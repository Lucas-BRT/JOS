use super::entity::User;
use crate::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &NewUser) -> Result<String>;
    async fn update(&self, user_id: &Uuid, data: &UpdateUser) -> Result<()>;
    async fn get_all(&self) -> Result<Vec<User>>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>>;
}
