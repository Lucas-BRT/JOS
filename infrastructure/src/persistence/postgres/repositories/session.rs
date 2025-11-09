use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::SessionModel;
use crate::persistence::postgres::models::session::ESessionStatus;
use domain::entities::*;
use domain::repositories::SessionRepository;
use shared::error::{ApplicationError, Error};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
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
    async fn create(&self, command: &CreateSessionCommand) -> Result<Session, Error> {
        let status = ESessionStatus::from(command.status);

        let created_session = sqlx::query_as!(
            SessionModel,
            r#"INSERT INTO sessions
                (
                id,
                title,
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
                title,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            "#,
            command.id,
            command.title,
            command.description,
            command.table_id,
            command.scheduled_for,
            status as ESessionStatus
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_session.into())
    }

    async fn read(&self, command: &GetSessionCommand) -> Result<Vec<Session>, Error> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                title,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            FROM sessions
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::text IS NULL OR title = $2)
              AND ($3::uuid IS NULL OR table_id = $3)
              AND ($4::timestamptz IS NULL OR scheduled_for >= $4)
              AND ($5::timestamptz IS NULL OR scheduled_for <= $5)
            "#,
            command.id,
            command.title,
            command.table_id,
            command.scheduled_for_start,
            command.scheduled_for_end
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: &UpdateSessionCommand) -> Result<Session, Error> {
        let has_title_update = command.title.is_some();
        let has_description_update = command.description.is_some();
        let has_scheduled_for_update = command.scheduled_for.is_some();
        let has_status_update = command.status.is_some();

        if !(has_title_update
            || has_description_update
            || has_scheduled_for_update
            || has_status_update)
        {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".into(),
            }));
        }

        let status = command.status.map(ESessionStatus::from);
        let scheduled_for = command.scheduled_for.flatten();

        let updated = sqlx::query_as!(
            SessionModel,
            r#"
            UPDATE sessions
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                scheduled_for = COALESCE($4, scheduled_for),
                status = COALESCE($5::session_status, status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, title, description, table_id,
                scheduled_for, status as "status: ESessionStatus",
                created_at, updated_at
            "#,
            command.id,
            command.title,
            command.description,
            scheduled_for,
            status as Option<ESessionStatus>
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(todo!())
    }

    async fn delete(&self, command: &DeleteSessionCommand) -> Result<Session, Error> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"DELETE FROM sessions
            WHERE id = $1
            RETURNING
                id,
                title,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            "#,
            command.table_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(todo!())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, Error> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                title,
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

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>, Error> {
        let sessions = sqlx::query_as!(
            SessionModel,
            r#"
            SELECT
                id,
                title,
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
