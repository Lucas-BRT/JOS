use crate::Result;
use crate::domain::user::dtos::CreateUserCommand;
use crate::domain::user::dtos::UpdateUserCommand;
use crate::domain::user::entity::User;
use crate::domain::user::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl<'a> PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<String> {
        todo!()
    }
    async fn update(&self, user_id: &Uuid, data: &UpdateUserCommand) -> Result<()> {
        todo!()
    }
    async fn get_all(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn find_by_username(&self, name: &str) -> Result<Option<User>> {
        todo!()
    }
}
