use crate::Result;
use crate::domain::user::dtos::UpdateUserCommand;
use crate::domain::user::{
    dtos::CreateUserCommand, entity::User, user_repository::UserRepository as UserRepositoryTrait,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
        todo!()
    }
    async fn update(&self, data: &UpdateUserCommand) -> Result<()> {
        todo!()
    }
    async fn get_all(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn find_by_username(&self, name: &str) -> Result<User> {
        todo!()
    }
    async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        todo!()
    }
    async fn find_by_email(&self, email: &str) -> Result<User> {
        todo!()
    }
    async fn delete(&self, user_id: &Uuid) -> Result<User> {
        todo!()
    }
}
