use crate::Result;
use crate::domain::game_system::{GameSystem, GameSystemRepository};
use crate::infrastructure::entities::prelude::TGameSystem;
use crate::infrastructure::repositories::{constraint_mapper, error::RepositoryError};
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresGameSystemRepository {
    pool: Arc<PgPool>,
}

impl PostgresGameSystemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GameSystemRepository for PostgresGameSystemRepository {
    async fn create(&self, name: &str) -> Result<GameSystem> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let result = sqlx::query_as!(
            TGameSystem,
            r#"
            INSERT INTO t_game_system (id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, created_at, updated_at
            "#,
            id,
            name,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>> {
        let result = sqlx::query_as!(
            TGameSystem,
            "SELECT id, name, created_at, updated_at FROM t_game_system WHERE name ILIKE $1",
            name
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(result.map(|m| m.into()))
    }

    async fn get_all(&self) -> Result<Vec<GameSystem>> {
        let result = sqlx::query_as::<_, TGameSystem>("SELECT id, name FROM t_game_system")
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(result.into_iter().map(|m| m.into()).collect())
    }
}
