use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::RefreshTokenRow;
use domain::entities::RefreshToken;
use domain::repositories::RefreshTokenRepository;
use shared::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create(&self, token: &RefreshToken) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO refresh_tokens (id, user_id, token, expires_at, created_at)
               VALUES ($1, $2, $3, $4, NOW())"#,
            token.id,
            token.user_id,
            token.token,
            token.expires_at
        )
        .execute(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let row = sqlx::query_as!(
            RefreshTokenRow,
            r#"SELECT id, user_id, token, expires_at, created_at
               FROM refresh_tokens WHERE token = $1"#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(row.map(|r| r.into()))
    }

    async fn delete_by_token(&self, token: &str) -> Result<()> {
        sqlx::query!(r#"DELETE FROM refresh_tokens WHERE token = $1"#, token)
            .execute(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;
        Ok(())
    }

    async fn delete_by_user(&self, user_id: &uuid::Uuid) -> Result<()> {
        sqlx::query!(r#"DELETE FROM refresh_tokens WHERE user_id = $1"#, user_id)
            .execute(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;
        Ok(())
    }
}
