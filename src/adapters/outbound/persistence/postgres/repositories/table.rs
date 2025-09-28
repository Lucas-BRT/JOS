use crate::Result;
use crate::adapters::outbound::postgres::RepositoryError;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::TableModel;
use crate::domain::entities::commands::*;
use crate::domain::entities::*;
use crate::domain::repositories::TableRepository;
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

pub struct PostgresTableRepository {
    pool: PgPool,
}

impl PostgresTableRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRepository for PostgresTableRepository {
    async fn create(&self, command: CreateTableCommand) -> Result<Table> {
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

        let created_table = sqlx::query_as!(
            TableModel,
            r#"INSERT INTO tables
                (
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            "#,
            uuid,
            command.gm_id,
            command.title,
            command.description,
            command.slots as i32,
            command.game_system_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table.into())
    }

    async fn update(&self, command: UpdateTableCommand) -> Result<Table> {
        let has_title_update = matches!(command.title, Update::Change(_));
        let has_description_update = matches!(command.description, Update::Change(_));
        let has_slots_update = matches!(command.slots, Update::Change(_));
        let has_game_system_update = matches!(command.game_system_id, Update::Change(_));

        if !has_title_update
            && !has_description_update
            && !has_slots_update
            && !has_game_system_update
        {
            return Err(crate::shared::Error::Persistence(
                RepositoryError::DatabaseError(sqlx::Error::RowNotFound),
            ));
        }

        let title_value = match &command.title {
            Update::Change(title) => Some(title.as_str()),
            Update::Keep => None,
        };

        let description_value = match &command.description {
            Update::Change(description) => Some(description.as_str()),
            Update::Keep => None,
        };

        let slots_value = match command.slots {
            Update::Change(slots) => Some(slots as i32),
            Update::Keep => None,
        };

        let game_system_id_value = match &command.game_system_id {
            Update::Change(game_system_id) => Some(*game_system_id),
            Update::Keep => None,
        };

        let updated_table = sqlx::query_as!(
            TableModel,
            r#"
            UPDATE tables 
            SET 
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                slots = COALESCE($4, slots),
                game_system_id = COALESCE($5, game_system_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            "#,
            command.id,
            title_value,
            description_value,
            slots_value,
            game_system_id_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
    }

    async fn delete(&self, command: DeleteTableCommand) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"DELETE FROM tables
                WHERE id = $1
                RETURNING
                    id,
                    gm_id,
                    title,
                    description,
                    slots,
                    game_system_id,
                    created_at,
                    updated_at
            "#,
            command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.into())
    }

    async fn read(&self, command: GetTableCommand) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            FROM tables
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::uuid IS NULL OR gm_id = $2)
              AND ($3::text IS NULL OR title = $3)
              AND ($4::uuid IS NULL OR game_system_id = $4)
              AND ($5::int4 IS NULL OR slots = $5)
            "#,
            command.id,
            command.gm_id,
            command.title,
            command.game_system_id,
            command.slots.map(|s| s as i32)
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Table>> {
        let search_pattern = format!("%{}%", query);
        let tables = sqlx::query_as!(
            TableModel,
            r#"SELECT *
                FROM tables
                WHERE title ILIKE $1 OR description ILIKE $1
                LIMIT 10
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Table>> {
        let table = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            FROM tables
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.map(|model| model.into()))
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            FROM tables
            WHERE id = $1
            "#,
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                game_system_id,
                created_at,
                updated_at
            FROM tables
            WHERE gm_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }
}
