use crate::Result;
use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::SessionModel;
use crate::adapters::outbound::postgres::models::session::ESessionStatus;
use crate::domain::entities::*;
use crate::domain::repositories::SessionRepository;
use crate::domain::utils::update::Update;
use sqlx::PgPool;

pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn create(&self, session: CreateSessionCommand) -> Result<Session> {
        let status: ESessionStatus = session.status.into();

        let created_session = sqlx::query_as!(
            SessionModel,
            r#"INSERT INTO sessions
                (
                name,
                description,
                table_id,
                scheduled_for,
                status)
            VALUES
                ($1, $2, $3, $4, $5)
            RETURNING
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            "#,
            session.name,
            session.description,
            session.table_id,
            session.scheduled_for,
            status as _,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_session.into())
    }

    async fn read(&self, command: GetSessionCommand) -> Result<Vec<Session>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
            id,
            name,
            description,
            table_id,
            scheduled_for,
            status as "status: ESessionStatus",
            created_at,
            updated_at
            FROM sessions "#,
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

        if let Some(id) = &command.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(name) = &command.name {
            push_filter_separator(&mut builder);
            builder.push("name = ");
            builder.push_bind(name);
        }

        if let Some(table_id) = &command.table_id {
            push_filter_separator(&mut builder);
            builder.push("table_id = ");
            builder.push_bind(table_id);
        }

        if let Some(scheduled_for_start) = &command.scheduled_for_start {
            push_filter_separator(&mut builder);
            builder.push("scheduled_for >= ");
            builder.push_bind(scheduled_for_start);
        }

        if let Some(scheduled_for_end) = &command.scheduled_for_end {
            push_filter_separator(&mut builder);
            builder.push("scheduled_for <= ");
            builder.push_bind(scheduled_for_end);
        }

        if let Some(status) = &command.status {
            push_filter_separator(&mut builder);
            builder.push("status = ");
            builder.push_bind(ESessionStatus::from(*status));
        }

        let sessions = builder
            .build_query_as::<SessionModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE sessions SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(name) = &command.name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Update::Change(description) = &command.description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        if let Update::Change(scheduled_for) = &command.scheduled_for {
            separated.push("scheduled_for = ");
            separated.push_bind_unseparated(scheduled_for);
        }

        if let Update::Change(status) = &command.status {
            separated.push("status = ");
            separated.push_bind_unseparated(ESessionStatus::from(*status));
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(
            r#" RETURNING
            id,
            name,
            description,
            table_id,
            scheduled_for,
            status as "status: ESessionStatus",
            created_at,
            updated_at
            "#,
        );

        let updated_table = builder
            .build_query_as::<SessionModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
    }

    async fn delete(&self, command: DeleteSessionCommand) -> Result<Session> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"DELETE FROM sessions
            WHERE id = $1
            RETURNING
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            "#,
            command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session.into())
    }
}
