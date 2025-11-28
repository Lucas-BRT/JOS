use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableModel;
use domain::entities::commands::*;
use domain::entities::{Table, TableStatus};
use domain::repositories::TableRepository;
use shared::Result;
use shared::error::{ApplicationError, Error};
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

#[derive(Clone)]
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
    async fn create(&self, command: &CreateTableCommand) -> Result<Table> {
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
                status,
                game_system_id,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                status,
                game_system_id,
                created_at,
                updated_at
            "#,
            uuid,
            command.gm_id,
            command.title,
            command.description,
            command.slots as i32,
            TableStatus::Active.to_string(),
            command.game_system_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table.into())
    }

    async fn update(&self, command: &UpdateTableCommand) -> Result<Table> {
        if command.title.is_none()
            && command.description.is_none()
            && command.slots.is_none()
            && command.game_system_id.is_none()
            && command.status.is_none()
        {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let title_value = command.title.as_deref();
        let description_value = command.description.as_deref();
        let slots_value = command.slots.map(|s| s as i32);
        let game_system_id_value = command.game_system_id;
        let status_value = command.status.as_ref().map(|s| s.to_string());

        let updated_table = sqlx::query_as!(
            TableModel,
            r#"
            UPDATE tables
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                slots = COALESCE($4, slots),
                game_system_id = COALESCE($5, game_system_id),
                status = COALESCE($6, status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                status,
                game_system_id,
                created_at,
                updated_at
            "#,
            command.id,
            title_value,
            description_value,
            slots_value,
            game_system_id_value,
            status_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
    }

    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table> {
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
                status,
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

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Table>> {
        let table = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                status,
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

    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                status,
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

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT
                id,
                gm_id,
                title,
                description,
                slots,
                status,
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

    async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Option<Table>> {
        let table = sqlx::query_as!(
            TableModel,
            "SELECT
                tables.id,
                tables.gm_id,
                tables.title,
                tables.description,
                tables.slots,
                tables.status,
                tables.game_system_id,
                tables.created_at,
                tables.updated_at
            FROM tables
            INNER JOIN sessions
            ON tables.id = sessions.table_id
            WHERE sessions.id = $1",
            session_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.map(|model| model.into()))
    }

    async fn get_all(&self) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"
            SELECT * FROM tables
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }
}
