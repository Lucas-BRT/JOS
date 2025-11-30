use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::RefreshTokenModel;
use domain::entities::*;
use domain::repositories::{RefreshTokenRepository, Repository};
use shared::Result;
use sqlx::PgPool;
use uuid::Uuid;

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
impl
    Repository<
        RefreshToken,
        CreateRefreshTokenCommand,
        UpdateRefreshTokenCommand,
        GetRefreshTokenCommand,
        DeleteRefreshTokenCommand,
    > for PostgresRefreshTokenRepository
{
    async fn create(&self, token: CreateRefreshTokenCommand) -> Result<RefreshToken> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            r#"
                INSERT INTO refresh_tokens
                    (id, user_id, token, expires_at)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    *
            "#,
            token.id,
            token.user_id,
            token.token,
            token.expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(refresh_token.into())
    }

    async fn read(&self, _: GetRefreshTokenCommand) -> Result<Vec<RefreshToken>> {
        todo!()
    }

    async fn update(&self, _: UpdateRefreshTokenCommand) -> Result<RefreshToken> {
        todo!()
    }

    async fn delete(&self, _: DeleteRefreshTokenCommand) -> Result<RefreshToken> {
        todo!()
    }

    async fn find_by_id(&self, _: uuid::Uuid) -> Result<Option<RefreshToken>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            r#"
                SELECT
                    *
                FROM refresh_tokens
                WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(refresh_token.map(|r| r.into()))
    }

    async fn delete_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            r#"
                DELETE FROM refresh_tokens
                WHERE token = $1
                RETURNING *
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(refresh_token.map(|r| r.into()))
    }

    async fn delete_by_user(&self, user_id: Uuid) -> Result<Vec<RefreshToken>> {
        let refresh_token = sqlx::query_as!(
            RefreshTokenModel,
            r#"
                DELETE FROM refresh_tokens
                WHERE user_id = $1
                RETURNING *
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;
        Ok(refresh_token.into_iter().map(|r| r.into()).collect())
    }
}
