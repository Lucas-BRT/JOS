use super::{dtos::NewUser, entity::User};
use crate::prelude::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &NewUser) -> AppResult<String>;
    async fn get_all(&self) -> AppResult<Vec<User>>;
    async fn find_by_username(&self, name: &str) -> AppResult<Option<User>>;
}
