use crate::Result;
use crate::adapters::outbound::postgres::models::SessionCheckinModel;
use crate::adapters::outbound::postgres::{RepositoryError, constraint_mapper};
use crate::domain::entities::*;
use crate::domain::repositories::SessionCheckinRepository;
use sqlx::PgPool;

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
impl SessionCheckinRepository for PostgresSessionCheckinRepository {
    async fn create(&self, command: CreateSessionCheckinCommand) -> Result<SessionCheckin> {
        let created_session_checkin = sqlx::query_as!(
            SessionCheckinModel,
            r#"INSERT INTO session_checkins
                (
                session_intent_id,
                attendance,
                notes)
            VALUES
                ($1, $2, $3)
            RETURNING
                *
            "#,
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
        let mut query = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                session_intent_id,
                attendance,
                notes,
                created_at,
                updated_at
            FROM session_checkins
            "#,
        );

        let mut conditions = Vec::new();

        if let Some(id) = &command.id {
            conditions.push("id = ");
            query.push_bind(id);
        }

        if let Some(session_intent_id) = &command.session_intent_id {
            conditions.push("session_intent_id = ");
            query.push_bind(session_intent_id);
        }

        if let Some(attendance) = &command.attendance {
            conditions.push("attendance = ");
            query.push_bind(attendance);
        }

        if !conditions.is_empty() {
            query.push(" WHERE ");
            for (i, condition) in conditions.iter().enumerate() {
                if i > 0 {
                    query.push(" AND ");
                }
                query.push(condition);
            }
        }

        let session_checkins = query
            .build_query_as::<SessionCheckinModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(session_checkins
            .into_iter()
            .map(|model| model.into())
            .collect())
    }

    async fn update(&self, command: UpdateSessionCheckinCommand) -> Result<SessionCheckin> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE session_checkins SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(session_intent_id) = &command.session_intent_id {
            separated.push("session_intent_id = ");
            separated.push_bind_unseparated(session_intent_id);
        }

        if let Update::Change(attendance) = &command.attendance {
            separated.push("attendance = ");
            separated.push_bind_unseparated(attendance);
        }

        if let Update::Change(notes) = &command.notes {
            separated.push("notes = ");
            separated.push_bind_unseparated(notes);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        builder.push(" RETURNING *");

        let updated_session_checkin = builder
            .build_query_as::<SessionCheckinModel>()
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
}
