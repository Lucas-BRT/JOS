use crate::{
    domain::user::{NewUser, User},
    infra::db::postgres::models::user::UserRow,
    prelude::AppResult,
};
use uuid::Uuid;

pub trait UserRepository {
    async fn create(&self, user: &NewUser) -> AppResult<String>;
    async fn update(&self, user: &User) -> Result<(), String>;
    async fn get_all(&self) -> Result<Vec<UserRow>, String>;

    async fn find_by_id(&self, id: &Uuid) -> Result<UserRow, String>;
    async fn find_by_username(&self, name: &str) -> Result<UserRow, String>;
}
