use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::SessionModel;
use crate::persistence::postgres::models::session::ESessionStatus;
use domain::entities::*;
use domain::repositories::SessionRepository;
use shared::Result;
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

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
        let status = ESessionStatus::from(session.status);
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

        let created_session = sqlx::query_as!(
            SessionModel,
            r#"INSERT INTO sessions
                (
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, NOW(), NOW())
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
            uuid,
            session.name,
            session.description,
            session.table_id,
            session.scheduled_for,
            status as ESessionStatus
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_session.into())
    }

    async fn read(&self, command: GetSessionCommand) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            FROM sessions
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::text IS NULL OR name = $2)
              AND ($3::uuid IS NULL OR table_id = $3)
              AND ($4::timestamptz IS NULL OR scheduled_for >= $4)
              AND ($5::timestamptz IS NULL OR scheduled_for <= $5)
            "#,
            command.id,
            command.name,
            command.table_id,
            command.scheduled_for_start,
            command.scheduled_for_end
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        let has_name_update = matches!(command.name, Update::Change(_));
        let has_description_update = matches!(command.description, Update::Change(_));
        let has_scheduled_for_update = matches!(command.scheduled_for, Update::Change(_));
        let has_status_update = matches!(command.status, Update::Change(_));

        if !has_name_update
            && !has_description_update
            && !has_scheduled_for_update
            && !has_status_update
        {
            return Err(shared::error::Error::Persistence(
                shared::error::PersistenceError::DatabaseError("Row not found".to_string()),
            ));
        }

        let name_value = match &command.name {
            Update::Change(name) => Some(name.as_str()),
            Update::Keep => None,
        };

        let description_value = match &command.description {
            Update::Change(description) => Some(description.as_str()),
            Update::Keep => None,
        };

        let scheduled_for_value = match &command.scheduled_for {
            Update::Change(scheduled_for) => scheduled_for.as_ref(),
            Update::Keep => None,
        };

        let status_value = match command.status {
            Update::Change(status) => Some(ESessionStatus::from(status)),
            Update::Keep => None,
        };

        let updated_session = if let Some(status) = status_value {
            sqlx::query_as!(
                SessionModel,
                r#"
                UPDATE sessions 
                SET 
                    name = COALESCE($2, name),
                    description = COALESCE($3, description),
                    scheduled_for = COALESCE($4, scheduled_for),
                    status = $5::session_status,
                    updated_at = NOW()
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
                command.id,
                name_value,
                description_value,
                scheduled_for_value,
                status as ESessionStatus,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?
        } else {
            sqlx::query_as!(
                SessionModel,
                r#"
                UPDATE sessions 
                SET 
                    name = COALESCE($2, name),
                    description = COALESCE($3, description),
                    scheduled_for = COALESCE($4, scheduled_for),
                    updated_at = NOW()
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
                command.id,
                name_value,
                description_value,
                scheduled_for_value
            )
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?
        };

        Ok(updated_session.into())
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            FROM sessions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session.map(|model| model.into()))
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                name,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            FROM sessions
            WHERE table_id = $1
            "#,
            table_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|model| model.into()).collect())
    }
}
