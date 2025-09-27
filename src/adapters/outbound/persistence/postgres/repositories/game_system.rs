use crate::Result;
use crate::adapters::outbound::postgres::RepositoryError;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::GameSystemModel;
use crate::domain::entities::*;
use crate::domain::repositories::GameSystemRepository;
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

    async fn update(&self, command: &mut UpdateGameSystemCommand) -> Result<GameSystem> {
        let has_name_update = matches!(command.name, Update::Change(_));

        if !has_name_update {
            return Err(crate::shared::Error::Persistence(
                RepositoryError::DatabaseError(sqlx::Error::RowNotFound),
            ));
        }

        let name_value = match &command.name {
            Update::Change(name) => Some(name.as_str()),
            Update::Keep => None,
        };

        let updated_game_system = sqlx::query_as!(
            GameSystemModel,
            r#"
            UPDATE game_systems 
            SET 
                name = COALESCE($2, name),
                created_at = NOW()
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
