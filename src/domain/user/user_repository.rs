use crate::{
    domain::user::{NewUser, User},
    infra::db::postgres::models::user::UserRow,
    prelude::AppResult,
};

pub trait UserRepository {
    async fn create(&self, user: &NewUser) -> AppResult<String>;
    async fn update(&self, user: &User) -> AppResult<()>;
    async fn get_all(&self) -> AppResult<Vec<UserRow>>;
    async fn find_by_username(&self, name: &str) -> AppResult<Option<UserRow>>;
}
