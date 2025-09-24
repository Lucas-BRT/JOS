use crate::Result;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::GameSystemModel;
use crate::domain::entities::*;
use crate::domain::repositories::GameSystemRepository;
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
    async fn create(&self, command: &mut CreateGameSystemCommand) -> Result<GameSystem> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"
            INSERT INTO game_systems (name)
            VALUES ($1)
            RETURNING *
            "#,
            command.name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn read(&self, command: &mut GetGameSystemCommand) -> Result<Vec<GameSystem>> {
        let result = sqlx::query_as!(
            GameSystemModel,
            "SELECT *
            FROM game_systems
            WHERE name ILIKE $1",
            command.name,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into_iter().map(|m| m.into()).collect())
    }

    async fn update(&self, command: &mut UpdateGameSystemCommand) -> Result<GameSystem> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE game_systems SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(name) = &command.name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(" RETURNING *");

        let updated_game_system = builder
            .build_query_as::<GameSystemModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_game_system.into())
    }

    async fn delete(&self, command: &mut DeleteGameSystemCommand) -> Result<GameSystem> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"DELETE FROM game_systems WHERE id = $1 RETURNING *"#,
            command.id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }
}
