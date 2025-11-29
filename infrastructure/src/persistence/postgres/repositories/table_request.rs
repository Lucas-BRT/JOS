use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableRequestModel;
use crate::persistence::postgres::models::table_request::ETableRequestStatus;
use domain::entities::*;
use domain::repositories::{Repository, TableRequestRepository};
use shared::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresTableRequestRepository {
    pool: PgPool,
}

impl PostgresTableRequestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl
    Repository<
        TableRequest,
        CreateTableRequestCommand,
        UpdateTableRequestCommand,
        GetTableRequestCommand,
        DeleteTableRequestCommand,
    > for PostgresTableRequestRepository
{
    async fn create(&self, command: CreateTableRequestCommand) -> Result<TableRequest> {
        let result = sqlx::query_as!(
            TableRequestModel,
            r#"
                INSERT INTO table_requests
                    (id, user_id, table_id, message, status)
                VALUES
                    ($1, $2, $3, $4, $5)
                RETURNING
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
            "#,
            command.id,
            command.user_id,
            command.table_id,
            command.message,
            ETableRequestStatus::from(command.status) as ETableRequestStatus,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn update(&self, command: UpdateTableRequestCommand) -> Result<TableRequest> {
        let response = sqlx::query_as!(
            TableRequestModel,
            r#"
                UPDATE table_requests
                SET
                    status = $2::request_status,
                    message = COALESCE($3, message),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
            "#,
            command.id,
            command.status.map(ETableRequestStatus::from) as Option<ETableRequestStatus>,
            command.message
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(response.into())
    }

    async fn delete(&self, command: DeleteTableRequestCommand) -> Result<TableRequest> {
        let table = sqlx::query_as!(
            TableRequestModel,
            r#"
                DELETE FROM table_requests
                WHERE id = $1
                RETURNING
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
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

    async fn read(&self, command: GetTableRequestCommand) -> Result<Vec<TableRequest>> {
        let requests = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE ($1::uuid IS NULL OR id = $1)
                    AND ($2::uuid IS NULL OR user_id = $2)
                    AND ($3::uuid IS NULL OR table_id = $3)
                    AND ($4::request_status IS NULL OR status = $4)
            "#,
            command.id,
            command.user_id,
            command.table_id,
            command.status.map(ETableRequestStatus::from) as Option<ETableRequestStatus>
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableRequest>> {
        let request = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(request.map(|model| model.into()))
    }
}

#[async_trait::async_trait]
impl TableRequestRepository for PostgresTableRequestRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableRequest>> {
        let requests = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableRequest>> {
        let requests = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE table_id = $1
            "#,
            &table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>> {
        let requests = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE status = $1
            "#,
            ETableRequestStatus::from(status) as ETableRequestStatus
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_user_and_table(
        &self,
        user_id: Uuid,
        table_id: Uuid,
    ) -> Result<Vec<TableRequest>> {
        let requests = sqlx::query_as!(
            TableRequestModel,
            r#"
                SELECT
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                FROM table_requests
                WHERE user_id = $1 AND table_id = $2
            "#,
            user_id,
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }
}
