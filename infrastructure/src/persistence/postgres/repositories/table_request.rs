use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::TableRequestModel;
use crate::persistence::postgres::models::table_request::ETableRequestStatus;
use domain::entities::*;
use domain::repositories::TableRequestRepository;
use shared::Result;
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

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
impl TableRequestRepository for PostgresTableRequestRepository {
    async fn create(&self, request_data: CreateTableRequestCommand) -> Result<TableRequest> {
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

        let result = sqlx::query_as!(
            TableRequestModel,
            r#"INSERT INTO table_requests
                    (
                    id,
                    user_id,
                    table_id,
                    message,
                    status,
                    created_at,
                    updated_at)
                VALUES
                    ($1, $2, $3, $4, $5, NOW(), NOW())
                RETURNING
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                "#,
            uuid,
            request_data.user_id,
            request_data.table_id,
            request_data.message,
            ETableRequestStatus::Pending as ETableRequestStatus,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn update(&self, update_data: UpdateTableRequestCommand) -> Result<TableRequest> {
        let has_status_update = matches!(update_data.status, Update::Change(_));
        let has_message_update = matches!(update_data.message, Update::Change(_));

        if !has_status_update && !has_message_update {
            return Err(shared::error::Error::Persistence(
                shared::error::PersistenceError::DatabaseError("Row not found".to_string()),
            ));
        }

        let status_value = match update_data.status {
            Update::Change(status) => Some(ETableRequestStatus::from(status)),
            Update::Keep => None,
        };

        let message_value = match &update_data.message {
            Update::Change(message) => message.as_ref().map(|m| m.as_str()),
            Update::Keep => None,
        };

        let updated_request = if let Some(status) = status_value {
            sqlx::query_as!(
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
                update_data.id,
                status as ETableRequestStatus,
                message_value
            )
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?
        } else {
            sqlx::query_as!(
                TableRequestModel,
                r#"
                UPDATE table_requests
                SET
                    message = COALESCE($2, message),
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
                update_data.id,
                message_value
            )
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?
        };

        Ok(updated_request.into())
    }

    async fn delete(&self, command: DeleteTableRequestCommand) -> Result<TableRequest> {
        let table = sqlx::query_as!(
            TableRequestModel,
            r#"DELETE FROM table_requests
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
            "#,
            command.id,
            command.user_id,
            command.table_id
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
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>> {
        let status = ETableRequestStatus::from(status);

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
            status as ETableRequestStatus
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|model| model.into()).collect())
    }
}
