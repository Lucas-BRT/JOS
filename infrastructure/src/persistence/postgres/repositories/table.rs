use crate::persistence::models::table::ETableStatus;
use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableModel;
use domain::entities::Table;
use domain::entities::commands::*;
use domain::repositories::{Repository, TableRepository};
use shared::Result;
use sqlx::PgPool;
use uuid::Uuid;

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
impl Repository<Table, CreateTableCommand, UpdateTableCommand, GetTableCommand, DeleteTableCommand>
    for PostgresTableRepository
{
    async fn create(&self, command: CreateTableCommand) -> Result<Table> {
        let created_table = sqlx::query_as!(
            TableModel,
            r#"
            INSERT INTO tables
                (id, gm_id, title, description, slots, game_system_id)
            VALUES
                ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                gm_id,
                title,
                description,
                slots,
                status as "status: ETableStatus",
                game_system_id,
                created_at,
                updated_at
            "#,
            command.id,
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
                    status as "status: ETableStatus",
                    game_system_id,
                    created_at,
                    updated_at
            "#,
            command.id,
            command.title.as_deref(),
            command.description.as_deref(),
            command.slots.map(|s| s as i32),
            command.game_system_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
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
                    status as "status: ETableStatus",
                    game_system_id,
                    created_at,
                    updated_at
                FROM tables
                WHERE ($1::uuid IS NULL OR id = $1)
                    AND ($2::uuid IS NULL OR gm_id = $2)
                    AND ($3::table_status IS NULL OR status = $3)
                    AND ($4::uuid IS NULL OR game_system_id = $4)
            "#,
            command.id,
            command.gm_id,
            command.status.map(ETableStatus::from) as Option<ETableStatus>,
            command.game_system_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|model| model.into()).collect())
    }

    async fn delete(&self, command: DeleteTableCommand) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"
                DELETE FROM tables
                WHERE id = $1
                RETURNING
                    id,
                    gm_id,
                    title,
                    description,
                    slots,
                    status as "status: ETableStatus",
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
                    status as "status: ETableStatus",
                    game_system_id,
                    created_at,
                    updated_at
                FROM tables
                WHERE id = $1
            "#,
            &id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.map(|model| model.into()))
    }
}

#[async_trait::async_trait]
impl TableRepository for PostgresTableRepository {
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
                    status as "status: ETableStatus",
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
                    status as "status: ETableStatus",
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
            r#"
                SELECT
                    tables.id,
                    tables.gm_id,
                    tables.title,
                    tables.description,
                    tables.slots,
                    tables.status as "status: ETableStatus",
                    tables.game_system_id,
                    tables.created_at,
                    tables.updated_at
                FROM tables
                INNER JOIN sessions
                ON tables.id = sessions.table_id
                WHERE sessions.id = $1
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.map(|model| model.into()))
    }
}
