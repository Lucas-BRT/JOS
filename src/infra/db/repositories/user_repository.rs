use crate::{
    domain::user::{NewUser, User},
    infra::db::postgres::models::user::UserRow,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &NewUser) -> Result<(), String>;
    async fn update(&self, user: &User) -> Result<(), String>;
    async fn get_all(&self) -> Result<Vec<UserRow>, String>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<UserRow>, String>;
    async fn find_by_username(&self, name: &str) -> Result<UserRow, String>;
}
