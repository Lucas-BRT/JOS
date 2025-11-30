use crate::persistence::constraint_mapper::map_database_error;
use crate::persistence::models::ESessionStatus;
use crate::persistence::models::SessionModel;
use crate::persistence::models::TableDetailsModel;
use crate::persistence::models::UserModel;
use crate::persistence::models::table::ETableStatus;
use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableModel;
use domain::entities::Table;
use domain::entities::TableDetails;
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

    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Option<Table>> {
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

    async fn find_details_by_id(&self, table_id: Uuid) -> Result<Option<TableDetails>> {
        let table_task = sqlx::query_as!(
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
                WHERE tables.id = $1
            "#,
            table_id
        )
        .fetch_optional(&self.pool);

        let users_task = sqlx::query_as!(
            UserModel,
            r#"
                SELECT
                    users.id,
                    users.email,
                    users.username,
                    users.created_at,
                    users.password,
                    users.updated_at
                FROM users
                INNER JOIN table_members
                ON users.id = table_members.user_id
                WHERE table_members.table_id = $1
            "#,
            table_id
        )
        .fetch_all(&self.pool);

        let session_task = sqlx::query_as!(
            SessionModel,
            r#"
                SELECT
                    sessions.id,
                    sessions.title,
                    sessions.table_id,
                    sessions.description,
                    sessions.scheduled_for,
                    sessions.status as "status: ESessionStatus",
                    sessions.created_at,
                    sessions.updated_at
                FROM sessions
                WHERE sessions.table_id = $1
            "#,
            table_id
        )
        .fetch_all(&self.pool);

        let response = tokio::try_join!(table_task, users_task, session_task);

        match response {
            Ok((Some(table), users, sessions)) => {
                let details = TableDetailsModel {
                    id: table.id,
                    gm_id: table.gm_id,
                    title: table.title,
                    description: table.description,
                    players: users,
                    slots: table.slots,
                    sessions,
                    status: table.status,
                    game_system_id: table.game_system_id,
                    created_at: table.created_at,
                    updated_at: table.updated_at,
                };

                Ok(Some(details.into()))
            }
            Ok((None, _, _)) => Ok(None),
            Err(err) => Err(map_database_error(err).into()),
        }
    }
}
