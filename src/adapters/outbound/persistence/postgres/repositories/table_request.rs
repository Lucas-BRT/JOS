use crate::Result;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::domain::table_request::dtos::*;
use crate::domain::table_request::entity::TableRequest;
use crate::domain::table_request::table_request_repository::TableRequestRepository;
use crate::domain::utils::update::Update;
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
impl TableRequestRepository for PostgresTableRequestRepository {
    async fn create(&self, request_data: &CreateTableRequestCommand) -> Result<TableRequest> {
        let result = sqlx::query_as!(
            TableRequestModel,
            r#"INSERT INTO table_requests
                    (
                    user_id,
                    table_id,
                    message,
                    status)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id,
                    user_id,
                    table_id,
                    message,
                    status as "status: ETableRequestStatus",
                    created_at,
                    updated_at
                "#,
            request_data.user_id,
            request_data.table_id,
            request_data.message,
            ETableRequestStatus::Pending as _,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(result.into())
    }

    async fn update(&self, update_data: &UpdateTableRequestCommand) -> Result<TableRequest> {
        let mut builder = sqlx::QueryBuilder::new(r#"UPDATE table_requests SET "#);

        let mut separated = builder.separated(", ");

        if let Update::Change(status) = &update_data.status {
            separated.push("status = ");
            separated.push_bind_unseparated(ETableRequestStatus::from(*status));
        }

        if let Update::Change(message) = &update_data.message {
            separated.push("message = ");
            separated.push_bind_unseparated(message);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(update_data.id);

        builder.push(
            r#" RETURNING
                id,
                user_id,
                table_id,
                message,
                status as "status: ETableRequestStatus",
                created_at,
                updated_at"#,
        );

        let updated_request = builder
            .build_query_as::<TableRequestModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_request.into())
    }

    async fn delete(&self, request_data: &DeleteTableRequestCommand) -> Result<TableRequest> {
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
            request_data.id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(table.into())
    }

    async fn get(&self, command: &GetTableRequestCommand) -> Result<Vec<TableRequest>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                user_id,
                table_id,
                message,
                status as "status: ETableRequestStatus",
                created_at,
                updated_at
            FROM table_requests"#,
        );

        let mut has_where = false;
        let mut push_filter_separator = |b: &mut sqlx::QueryBuilder<'_, sqlx::Postgres>| {
            if !has_where {
                b.push(" WHERE ");
                has_where = true;
            } else {
                b.push(" AND ");
            }
        };

        if let Some(id) = &command.filters.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(user_id) = &command.filters.user_id {
            push_filter_separator(&mut builder);
            builder.push("user_id = ");
            builder.push_bind(user_id);
        }

        if let Some(table_id) = &command.filters.table_id {
            push_filter_separator(&mut builder);
            builder.push("table_id = ");
            builder.push_bind(table_id);
        }

        if let Some(status) = &command.filters.status {
            push_filter_separator(&mut builder);
            builder.push("status = ");
            builder.push_bind(ETableRequestStatus::from(*status));
        }

        let page = command.pagination.limit();
        let offset = command.pagination.offset();

        builder.push(" LIMIT ");
        builder.push_bind(page as i64);

        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);

        let requests = builder
            .build_query_as::<TableRequestModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(requests.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<TableRequest> {
        let request = sqlx::query_as!(
            TableRequestModel,
            r#"SELECT
                id,
                user_id,
                table_id,
                message,
                status as "status: ETableRequestStatus",
                created_at,
                updated_at
            FROM table_requests WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(request.into())
    }
}
