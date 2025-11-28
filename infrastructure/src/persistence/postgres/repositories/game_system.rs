use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::GameSystemModel;
use domain::entities::*;
use domain::repositories::{GameSystemRepository, Repository};
use shared::Result;
use shared::error::{ApplicationError, Error};
use sqlx::PgPool;
use uuid::Uuid;

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
impl
    Repository<
        GameSystem,
        CreateGameSystemCommand,
        UpdateGameSystemCommand,
        GetGameSystemCommand,
        DeleteGameSystemCommand,
    > for PostgresGameSystemRepository
{
    async fn create(&self, command: CreateGameSystemCommand) -> Result<GameSystem> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"
                INSERT INTO game_systems
                    (id,
                    name,
                    created_at,
                    updated_at)
                VALUES
                    ($1,
                    $2,
                    NOW(),
                    NOW())
                RETURNING *
                "#,
            command.id,
            command.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn read(&self, command: GetGameSystemCommand) -> Result<Vec<GameSystem>> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"
                SELECT *
                FROM game_systems
                WHERE ($1::text IS NULL OR name ILIKE $1)
                "#,
            command.name.as_ref().map(|s| format!("%{}%", s))
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into_iter().map(|m| m.into()).collect())
    }

    async fn update(&self, command: UpdateGameSystemCommand) -> Result<GameSystem> {
        if command.name.is_none() {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let name_value = command.name.as_deref();

        let updated_game_system = sqlx::query_as!(
            GameSystemModel,
            r#"
                UPDATE game_systems
                SET
                    name = COALESCE($2, name),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *
                "#,
            command.id,
            name_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_game_system.into())
    }

    async fn delete(&self, command: DeleteGameSystemCommand) -> Result<GameSystem> {
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>> {
        let game_system = sqlx::query_as!(
            GameSystemModel,
            r#"
                SELECT *
                FROM game_systems
                WHERE id = $1
                "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(game_system.map(|model| model.into()))
    }
}

#[async_trait::async_trait]
impl GameSystemRepository for PostgresGameSystemRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>> {
        let game_system = sqlx::query_as!(
            GameSystemModel,
            r#"
            SELECT *
            FROM game_systems
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(game_system.map(|model| model.into()))
    }
}
