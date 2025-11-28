use crate::persistence::postgres::models::SessionCheckinModel;
use crate::persistence::postgres::{RepositoryError, constraint_mapper};
use domain::entities::*;
use domain::repositories::{Repository, SessionCheckinRepository};
use shared::Result;
use shared::error::{ApplicationError, Error};
use sqlx::PgPool;
use uuid::{NoContext, Uuid};

#[derive(Clone)]
pub struct PostgresSessionCheckinRepository {
    pool: PgPool,
}

impl PostgresSessionCheckinRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl
    Repository<
        SessionCheckin,
        CreateSessionCheckinCommand,
        UpdateSessionCheckinCommand,
        GetSessionCheckinCommand,
        DeleteSessionCheckinCommand,
    > for PostgresSessionCheckinRepository
{
    async fn create(&self, command: CreateSessionCheckinCommand) -> Result<SessionCheckin> {
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

        let created_session_checkin = sqlx::query_as!(
            SessionCheckinModel,
            r#"INSERT INTO session_checkins
                (
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, $4, NOW(), NOW())
            RETURNING
                *
            "#,
            uuid,
            &command.session_intent_id,
            &command.attendance,
            command.notes.as_deref(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_session_checkin.into())
    }

    async fn read(&self, command: GetSessionCheckinCommand) -> Result<Vec<SessionCheckin>> {
        let session_checkins = sqlx::query_as!(
            SessionCheckinModel,
            r#"
            SELECT
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at
            FROM session_checkins
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::uuid IS NULL OR session_intent_id = $2)
              AND ($3::bool IS NULL OR attendance = $3)
            "#,
            command.id,
            command.session_intent_id,
            command.attendance
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkins
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn update(&self, command: UpdateSessionCheckinCommand) -> Result<SessionCheckin> {
        if command.session_intent_id.is_none()
            && command.attendance.is_none()
            && command.notes.is_none()
        {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let session_intent_id_value = command.session_intent_id;
        let attendance_value = command.attendance;
        let notes_value = command.notes.as_ref().and_then(|n| n.as_deref());

        let updated_session_checkin = sqlx::query_as!(
            SessionCheckinModel,
            r#"
            UPDATE session_checkins
            SET
                session_intent_id = COALESCE($2, session_intent_id),
                attendance = COALESCE($3, attendance),
                notes = COALESCE($4, notes),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            command.id,
            session_intent_id_value,
            attendance_value,
            notes_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_session_checkin.into())
    }

    async fn delete(&self, command: DeleteSessionCheckinCommand) -> Result<SessionCheckin> {
        let session_checkin = sqlx::query_as!(
            SessionCheckinModel,
            r#"DELETE FROM session_checkins
            WHERE id = $1
            RETURNING
                *
            "#,
            &command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkin.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionCheckin>> {
        let session_checkin = sqlx::query_as!(
            SessionCheckinModel,
            r#"
            SELECT
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at
            FROM session_checkins
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkin.map(|model| model.into()))
    }
}

#[async_trait::async_trait]
impl SessionCheckinRepository for PostgresSessionCheckinRepository {
    async fn find_by_session_intent_id(
        &self,
        session_intent_id: Uuid,
    ) -> Result<Vec<SessionCheckin>> {
        let session_checkins = sqlx::query_as!(
            SessionCheckinModel,
            r#"
            SELECT
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at
            FROM session_checkins
            WHERE session_intent_id = $1
            "#,
            session_intent_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkins
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>> {
        let session_checkins = sqlx::query_as!(
            SessionCheckinModel,
            r#"
            SELECT
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at
            FROM session_checkins
            WHERE attendance = $1
            "#,
            attendance
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkins
            .into_iter()
            .map(|model| model.into())
            .collect())
    }
}
