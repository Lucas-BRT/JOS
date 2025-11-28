use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::SessionModel;
use crate::persistence::postgres::models::session::ESessionStatus;
use domain::entities::session_checkin::{SessionFinalizationData, SessionFinalizationResult};
use domain::entities::*;
use domain::repositories::SessionRepository;
use shared::Result;
use shared::error::{ApplicationError, Error};
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
        let status = ESessionStatus::Scheduled; // Default status for new sessions
        let uuid = Uuid::new_v7(uuid::Timestamp::now(NoContext));

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
            uuid,
            session.title,
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
                title,
                description,
                table_id,
                scheduled_for,
                status as "status: ESessionStatus",
                created_at,
                updated_at
            FROM sessions
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::uuid IS NULL OR table_id = $2)
              AND ($3::timestamptz IS NULL OR scheduled_for >= $3)
              AND ($4::timestamptz IS NULL OR scheduled_for <= $4)
            "#,
            command.id,
            command.table_id,
            command.scheduled_after,
            command.scheduled_before
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        if command.title.is_none()
            && command.description.is_none()
            && command.scheduled_for.is_none()
            && command.status.is_none()
        {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let title_value = command.title.as_ref().map(|s| s.as_str());
        let description_value = command.description.as_ref().map(|s| s.as_str());
        let scheduled_for_value = command.scheduled_for.as_ref();
        let status_value = command.status.map(ESessionStatus::from);

        let updated_session = if let Some(status) = status_value {
            sqlx::query_as!(
                SessionModel,
                r#"
                UPDATE sessions
                SET
                    title = COALESCE($2, title),
                    description = COALESCE($3, description),
                    scheduled_for = COALESCE($4, scheduled_for),
                    status = $5::session_status,
                    updated_at = NOW()
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
                command.id,
                title_value,
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
                    title = COALESCE($2, title),
                    description = COALESCE($3, description),
                    scheduled_for = COALESCE($4, scheduled_for),
                    updated_at = NOW()
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
                command.id,
                title_value,
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
                title,
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

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>> {
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

    async fn finalize_session_with_checkins(
        &self,
        finalization_data: SessionFinalizationData,
    ) -> Result<SessionFinalizationResult> {
        todo!()
    }
}
