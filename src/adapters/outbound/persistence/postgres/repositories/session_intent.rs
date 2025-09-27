use crate::{
    Error, Result,
    adapters::outbound::postgres::{
        constraint_mapper,
        models::{SessionIntentModel, session_intent::EIntentStatus},
    },
    domain::{
        entities::{
            CreateSessionIntentCommand, DeleteSessionIntentCommand, GetSessionIntentCommand,
            SessionIntent, Update, UpdateSessionIntentCommand,
        },
        error::SessionIntentDomainError,
        repositories::SessionIntentRepository,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresSessionIntentRepository {
    pool: PgPool,
}

impl PostgresSessionIntentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionIntentRepository for PostgresSessionIntentRepository {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent> {
        let session_intent = sqlx::query_as!(
            SessionIntentModel,
            r#"INSERT INTO session_intents
            (
                user_id,
                session_id,
                intent_status)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            "#,
            command.player_id,
            command.session_id,
            EIntentStatus::from(command.status) as _,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intent.into())
    }

    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent> {
        let status_to_update = matches!(command.status, Update::Change(_));

        if !status_to_update {
            let session_intent = self.find_by_id(command.id).await?;

            match session_intent {
                Some(session_intent) => return Ok(session_intent),
                None => {
                    return Err(Error::Domain(
                        SessionIntentDomainError::SessionIntentNotFound.into(),
                    ));
                }
            }
        }

        let new_status: Option<EIntentStatus> = match command.status {
            Update::Change(status) => Some(status.into()),
            Update::Keep => None,
        };

        let updated_model = sqlx::query_as!(
            SessionIntentModel,
            r#"
                UPDATE session_intents 
                SET 
                    intent_status = COALESCE($2, intent_status),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING
                    id,
                    user_id,
                    session_id,
                    intent_status as "intent_status: EIntentStatus",
                    created_at,
                    updated_at
            "#,
            command.id,
            new_status as Option<EIntentStatus>
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_model.into())
    }

    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent> {
        let session_intent = sqlx::query_as!(
            SessionIntentModel,
            r#"DELETE FROM session_intents
            WHERE id = $1
            RETURNING
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            "#,
            command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intent.into())
    }

    async fn read(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>> {
        let sessions = sqlx::query_as!(
            SessionIntentModel,
            r#"
            SELECT
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            FROM session_intents
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::uuid IS NULL OR user_id = $2)
              AND ($3::uuid IS NULL OR session_id = $3)
            "#,
            command.id,
            command.user_id,
            command.session_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>> {
        let session_intent = sqlx::query_as!(
            SessionIntentModel,
            r#"
            SELECT
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            FROM session_intents
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intent.map(|model| model.into()))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SessionIntent>> {
        let session_intents = sqlx::query_as!(
            SessionIntentModel,
            r#"
            SELECT
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            FROM session_intents
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intents
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<SessionIntent>> {
        let session_intents = sqlx::query_as!(
            SessionIntentModel,
            r#"
            SELECT
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            FROM session_intents
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intents
            .into_iter()
            .map(|model| model.into())
            .collect())
    }
}
