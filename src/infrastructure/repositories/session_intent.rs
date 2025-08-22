use crate::Result;
use crate::domain::session_intent::{
    CreateSessionIntentCommand, DeleteSessionIntentCommand, GetSessionIntentCommand, SessionIntent,
    SessionIntentRepository, UpdateSessionIntentCommand,
};
use crate::domain::utils::update::Update;
use crate::infrastructure::entities::{EIntentStatus, SessionIntentModel};
use crate::infrastructure::repositories::constraint_mapper;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresSessionIntentRepository {
    pool: Arc<PgPool>,
}

impl PostgresSessionIntentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionIntentRepository for PostgresSessionIntentRepository {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let session_intent = sqlx::query_as!(
            SessionIntentModel,
            r#"INSERT INTO t_session_intents
                (id,
                user_id,
                session_id,
                intent_status,
                created_at,
                updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                user_id,
                session_id,
                intent_status as "intent_status: EIntentStatus",
                created_at,
                updated_at
            "#,
            id,
            command.player_id,
            command.session_id,
            EIntentStatus::from(command.status) as _,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intent.into())
    }
    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent> {
        let now = Utc::now();

        let mut builder = sqlx::QueryBuilder::new("UPDATE t_session_intents SET ");
        let mut separated = builder.separated(" ,");

        if let Update::Change(status) = command.status {
            separated.push("intent_status = ");
            separated.push_bind_unseparated(EIntentStatus::from(status));
        }

        separated.push(" updated_at = ");
        separated.push_bind_unseparated(now);

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(r#" RETURNING *"#);

        let updated_session_intent = builder
            .build_query_as::<SessionIntentModel>()
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_session_intent.into())
    }
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent> {
        let session_intent = sqlx::query_as!(
            SessionIntentModel,
            r#"DELETE FROM t_session_intents
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
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(session_intent.into())
    }
    async fn get(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>> {
        let mut builder = sqlx::QueryBuilder::new(r#"SELECT * FROM t_session_intents "#);

        let mut has_where = false;

        let mut push_filter_separator = |b: &mut sqlx::QueryBuilder<'_, sqlx::Postgres>| {
            if !has_where {
                b.push(" WHERE ");
                has_where = true;
            } else {
                b.push(" AND ");
            }
        };

        if let Some(id) = command.filters.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(user_id) = command.filters.user_id {
            push_filter_separator(&mut builder);
            builder.push("user_id = ");
            builder.push_bind(user_id);
        }

        if let Some(session_id) = command.filters.session_id {
            push_filter_separator(&mut builder);
            builder.push("session_id = ");
            builder.push_bind(session_id);
        }

        if let Some(intent_status) = command.filters.intent_status {
            push_filter_separator(&mut builder);
            builder.push("intent_status = ");
            builder.push_bind(EIntentStatus::from(intent_status));
        }

        let page = command.pagination.limit();
        let offset = command.pagination.offset();

        builder.push(" LIMIT ");
        builder.push_bind(page as i64);

        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);

        let sessions = builder
            .build_query_as::<SessionIntentModel>()
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(sessions.into_iter().map(|s| s.into()).collect())
    }
}
