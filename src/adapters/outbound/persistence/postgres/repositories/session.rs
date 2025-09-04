use crate::Result;
use crate::domain::session::{
    CreateSessionCommand, DeleteSessionCommand, GetSessionCommand, Session, SessionRepository,
    UpdateSessionCommand,
};
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
    async fn create(&self, session: &CreateSessionCommand) -> Result<Session> {
        let status: ESessionStatus = session.status.into();

        let created_session = sqlx::query_as!(
            SessionModel,
            r#"INSERT INTO sessions
                (
                name,
                description,
                table_id,
                scheduled_for,
                status,
                accepting_intents)
            VALUES
                ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                accepting_intents,
                created_at,
                updated_at
            "#,
            session.name,
            session.description,
            session.table_id,
            session.scheduled_for,
            status as _,
            session.accepting_intents,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_session.into())
    }

    async fn get(&self, command: &GetSessionCommand) -> Result<Vec<Session>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
            id,
            name,
            description,
            table_id,
            scheduled_for,
            status as "status: ESessionStatus",
            accepting_intents,
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

        if let Some(id) = &command.filters.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(name) = &command.filters.name {
            push_filter_separator(&mut builder);
            builder.push("name = ");
            builder.push_bind(name);
        }

        if let Some(description) = &command.filters.description {
            push_filter_separator(&mut builder);
            builder.push("description = ");
            builder.push_bind(description);
        }

        if let Some(accepting_intents) = &command.filters.accepting_intents {
            push_filter_separator(&mut builder);
            builder.push("accepting_intents = ");
            builder.push_bind(accepting_intents);
        }

        if let Some(table_id) = &command.filters.table_id {
            push_filter_separator(&mut builder);
            builder.push("table_id = ");
            builder.push_bind(table_id);
        }

        if let Some(created_at) = &command.filters.created_at {
            push_filter_separator(&mut builder);
            builder.push("created_at = ");
            builder.push_bind(created_at);
        }

        if let Some(updated_at) = &command.filters.updated_at {
            push_filter_separator(&mut builder);
            builder.push("updated_at = ");
            builder.push_bind(updated_at);
        }

        let page = command.pagination.limit();
        let offset = command.pagination.offset();

        builder.push(" LIMIT ");
        builder.push_bind(page as i64);

        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);

        let sessions = builder
            .build_query_as::<SessionModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Session>> {
        let session: Option<Session> = sqlx::query_as!(
            SessionModel,
            r#"SELECT
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                accepting_intents,
                created_at,
                updated_at
            FROM sessions
            WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?
        .map(Session::from);

        Ok(session)
    }

    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"SELECT
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                accepting_intents,
                created_at,
                updated_at
            FROM sessions
            WHERE table_id = $1"#,
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: &UpdateSessionCommand) -> Result<Session> {
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

        if let Update::Change(accepting_intents) = &command.accepting_intents {
            separated.push("accepting_intents = ");
            separated.push_bind_unseparated(accepting_intents);
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
            accepting_intents,
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

    async fn delete(&self, command: &DeleteSessionCommand) -> Result<Session> {
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
                accepting_intents,
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
