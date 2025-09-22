use crate::Result;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::domain::game_system::{GameSystem, GameSystemRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresGameSystemRepository {
    pool: PgPool,
}

impl PostgresGameSystemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl GameSystemRepository for PostgresGameSystemRepository {
    async fn create(&self, name: &str) -> Result<GameSystem> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"
            INSERT INTO game_systems (name)
            VALUES ($1)
            RETURNING *
            "#,
            name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>> {
        let result = sqlx::query_as!(
            GameSystemModel,
            "SELECT *
            FROM game_systems
            WHERE name ILIKE $1",
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.map(|m| m.into()))
    }

    async fn get_all(&self) -> Result<Vec<GameSystem>> {
        let result = sqlx::query_as!(GameSystemModel, "SELECT * FROM game_systems")
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into_iter().map(|m| m.into()).collect())
    }
}
