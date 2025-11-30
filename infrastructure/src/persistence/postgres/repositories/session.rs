use crate::persistence::models::{EIntentStatus, SessionCheckinResultModel};
use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::SessionModel;
use crate::persistence::postgres::models::session::ESessionStatus;
use domain::entities::session_checkin::{SessionFinalizationData, SessionFinalizationResult};
use domain::entities::*;
use domain::repositories::{Repository, SessionRepository};
use shared::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl
    Repository<
        Session,
        CreateSessionCommand,
        UpdateSessionCommand,
        GetSessionCommand,
        DeleteSessionCommand,
    > for PostgresSessionRepository
{
    async fn create(&self, session: CreateSessionCommand) -> Result<Session> {
        let created_session = sqlx::query_as!(
            SessionModel,
            r#"
                INSERT INTO sessions
                    (id, title, description, table_id, scheduled_for, status)
                VALUES
                    ($1, $2, $3, $4, $5, $6)
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
            session.id,
            session.title,
            session.description,
            session.table_id,
            session.scheduled_for.as_ref(),
            ESessionStatus::from(session.status) as ESessionStatus
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
                    AND ($5::session_status IS NULL OR status = $5)
            "#,
            command.id,
            command.table_id,
            command.scheduled_after.as_ref(),
            command.scheduled_before.as_ref(),
            command.status.map(ESessionStatus::from) as Option<ESessionStatus>,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn update(&self, command: UpdateSessionCommand) -> Result<Session> {
        let updated_session = sqlx::query_as!(
            SessionModel,
            r#"
                UPDATE sessions
                SET
                    title = COALESCE($2, title),
                    description = COALESCE($3, description),
                    scheduled_for = COALESCE($4, scheduled_for),
                    status = COALESCE($5, status),
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
            command.title.as_deref(),
            command.description.as_deref(),
            command.scheduled_for,
            command.status.map(ESessionStatus::from) as Option<ESessionStatus>,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_session.into())
    }

    async fn delete(&self, command: DeleteSessionCommand) -> Result<Session> {
        let session = sqlx::query_as!(
            SessionModel,
            r#"
                DELETE FROM sessions
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
}

#[async_trait::async_trait]
impl SessionRepository for PostgresSessionRepository {
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
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(constraint_mapper::map_database_error)?;

        let checkins_len = finalization_data.checkins.len();
        let mut user_ids = Vec::with_capacity(checkins_len);
        let mut attendances = Vec::with_capacity(checkins_len);
        let mut notes = Vec::with_capacity(checkins_len);

        for checkin in &finalization_data.checkins {
            user_ids.push(checkin.user_id);
            attendances.push(checkin.attendance);
            notes.push(checkin.notes.clone());
        }

        let checkins = sqlx::query_as!(
            SessionCheckinResultModel,
            r#"
                WITH input_data AS (
                    SELECT *
                    FROM UNNEST($1::uuid[], $2::boolean[], $3::text[])
                    AS t(user_id, attendance, notes)
                ),
                matched_intents AS (
                    SELECT
                        si.id as session_intent_id,
                        d.user_id,
                        si.intent_status,
                        d.attendance,
                        d.notes
                    FROM input_data d
                    JOIN session_intents si
                    ON si.user_id = d.user_id
                    AND si.session_id = $4
                ),
                inserted_checkins AS (
                    INSERT INTO session_checkins
                        (session_intent_id, attendance, notes)
                    SELECT session_intent_id, attendance, notes
                    FROM matched_intents
                    RETURNING id, session_intent_id, attendance
                )
                SELECT
                    mi.user_id as "user_id!: Uuid",
                    mi.intent_status as "intent_status!: EIntentStatus",
                    ic.attendance as "attendance!: bool",
                    ic.id as "checkin_id!: Uuid"
                FROM inserted_checkins ic
                JOIN matched_intents mi ON ic.session_intent_id = mi.session_intent_id
            "#,
            &user_ids,
            &attendances,
            &notes as &[Option<String>],
            finalization_data.session_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        let updated_session = sqlx::query_as!(
            SessionModel,
            r#"
            UPDATE sessions
            SET
                status = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, title, description, table_id, scheduled_for,
                status as "status: _", created_at, updated_at
            "#,
            finalization_data.session_id,
            ESessionStatus::Completed as ESessionStatus,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        tx.commit()
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(SessionFinalizationResult {
            session: updated_session.into(),
            checkins: checkins.into_iter().map(|c| c.into()).collect(),
        })
    }
}
