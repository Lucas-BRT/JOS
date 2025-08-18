use crate::Result;
use crate::domain::table_request::dtos::*;
use crate::domain::table_request::entity::TableRequest;
use crate::domain::table_request::table_request_repository::TableRequestRepository;
use crate::domain::utils::pagination::Pagination;
use crate::infrastructure::entities::enums::ETableRequestStatus;
use crate::infrastructure::entities::t_table_requests::Model as TableRequestModel;
use crate::infrastructure::prelude::RepositoryError;
use crate::infrastructure::repositories::constraint_mapper;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresTableRequestRepository {
    pool: Arc<PgPool>,
}

impl PostgresTableRequestRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRequestRepository for PostgresTableRequestRepository {
    async fn create(&self, request_data: &mut CreateTableRequestCommand) -> Result<TableRequest> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let result = sqlx::query_as!(
            TableRequestModel,
            r#"INSERT INTO t_table_requests 
                    (id, 
                    user_id, 
                    table_id, 
                    message,
                    status,
                    created_at,
                    updated_at)
                VALUES
                    ($1, 
                    $2, 
                    $3, 
                    $4, 
                    $5, 
                    $6, 
                    $7)
                RETURNING 
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                "#,
            id,
            request_data.user_id,
            request_data.table_id,
            request_data.message,
            ETableRequestStatus::Pending as _,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn update(&self, _update_data: &UpdateTableRequestCommand) -> Result<()> {
        todo!()
    }

    async fn delete(&self, request_data: &DeleteTableRequestCommand) -> Result<TableRequest> {
        let table = sqlx::query_as!(
            TableRequestModel,
            r#"SELECT 
                id,
                user_id,
                table_id,
                message,
                status as "status: ETableRequestStatus",
                created_at,
                updated_at
            FROM t_table_requests WHERE id = $1"#,
            request_data.id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        match table {
            Some(table) => {
                sqlx::query(
                    r#"DELETE FROM t_table_requests 
                        WHERE id = $1"#,
                )
                .bind(request_data.id)
                .execute(self.pool.as_ref())
                .await
                .map_err(constraint_mapper::map_database_error)?;

                Ok(table.into())
            }
            None => {
                return Err(RepositoryError::TableRequestNotFound.into());
            }
        }
    }

    async fn get(
        &self,
        _filters: &TableRequestFilters,
        _pagination: Pagination,
    ) -> Result<Vec<TableRequest>> {
        todo!()
    }

    async fn find(&self, _filters: &TableRequestFilters) -> Result<TableRequest> {
        todo!()
    }
}
