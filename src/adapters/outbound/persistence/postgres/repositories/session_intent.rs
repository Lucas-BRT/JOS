use crate::{
    Result,
    adapters::outbound::postgres::{
        constraint_mapper,
        models::{SessionIntentModel, session_intent::EIntentStatus},
    },
    domain::{
        entities::{
            CreateSessionIntentCommand, DeleteSessionIntentCommand, GetSessionIntentCommand,
            SessionIntent, Update, UpdateSessionIntentCommand,
        },
        repositories::SessionIntentRepository,
    },
};
use sqlx::PgPool;

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
        let mut builder = sqlx::QueryBuilder::new("UPDATE session_intents SET ");
        let mut separated = builder.separated(" ,");

        if let Update::Change(status) = command.status {
            separated.push("intent_status = ");
            separated.push_bind_unseparated(EIntentStatus::from(status));
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(
            r#" RETURNING
            id,
            user_id,
            session_id,
            intent_status as "intent_status: EIntentStatus",
            created_at,
            updated_at
            "#,
        );

        let updated_session_intent = builder
            .build_query_as::<SessionIntentModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_session_intent.into())
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
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT
            id,
            user_id,
            session_id,
            intent_status as "intent_status: EIntentStatus",
            created_at,
            updated_at
            FROM session_intents "#,
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

        if let Some(id) = command.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(user_id) = command.user_id {
            push_filter_separator(&mut builder);
            builder.push("user_id = ");
            builder.push_bind(user_id);
        }

        if let Some(session_id) = command.session_id {
            push_filter_separator(&mut builder);
            builder.push("session_id = ");
            builder.push_bind(session_id);
        }

        if let Some(intent_status) = command.status {
            push_filter_separator(&mut builder);
            builder.push("intent_status = ");
            builder.push_bind(EIntentStatus::from(intent_status));
        }

        let sessions = builder
            .build_query_as::<SessionIntentModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }
}
