use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::GameSystemModel;
use domain::entities::game_system::GameSystemBuilder;
use domain::entities::*;
use domain::repositories::GameSystemRepository;
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
impl GameSystemRepository for PostgresGameSystemRepository {
    async fn create(&self, command: &CreateGameSystemCommand) -> Result<GameSystem, Error> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"
            INSERT INTO game_systems (id, name, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            RETURNING *
            "#,
            command.id,
            command.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        let game_system = GameSystemBuilder::default()
            .with_id(result.id)
            .with_name(result.name)
            .build()?;

        Ok(game_system)
    }

    async fn read(&self, command: &GetGameSystemCommand) -> Result<Vec<GameSystem>, Error> {
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

        let systems = result
            .into_iter()
            .map(|gs| {
                GameSystemBuilder::default()
                    .with_id(gs.id)
                    .with_name(gs.name)
                    .build()
            })
            .collect::<Result<Vec<GameSystem>, Error>>()?;

        Ok(systems)
    }

    async fn update(&self, command: &UpdateGameSystemCommand) -> Result<GameSystem, Error> {
        let has_name_update = command.name.is_some();

        if !has_name_update {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let result = sqlx::query_as!(
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
            command.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        let system = GameSystemBuilder::default()
            .with_id(result.id)
            .with_name(result.name)
            .build()?;

        Ok(system)
    }

    async fn delete(&self, command: &DeleteGameSystemCommand) -> Result<GameSystem, Error> {
        let result = sqlx::query_as!(
            GameSystemModel,
            r#"DELETE FROM game_systems WHERE id = $1 RETURNING *"#,
            command.id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        let system = GameSystemBuilder::default()
            .with_id(result.id)
            .with_name(result.name)
            .build()?;

        Ok(system)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>, Error> {
        let result = sqlx::query_as!(
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

        match result {
            Some(gs) => {
                let system = GameSystemBuilder::default()
                    .with_id(gs.id)
                    .with_name(gs.name)
                    .build()?;
                Ok(Some(system))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>, Error> {
        let result = sqlx::query_as!(
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

        match result {
            Some(gs) => {
                let system = GameSystemBuilder::default()
                    .with_id(gs.id)
                    .with_name(gs.name)
                    .build()?;
                Ok(Some(system))
            }
            None => Ok(None),
        }
    }
}
