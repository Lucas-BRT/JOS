use crate::{entities::*, repositories::Repository};
use async_trait::async_trait;
use shared::Result;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokenRepository:
    Repository<
        RefreshToken,
        CreateRefreshTokenCommand,
        UpdateRefreshTokenCommand,
        GetRefreshTokenCommand,
        DeleteRefreshTokenCommand,
    > + Send
    + Sync
{
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    async fn delete_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    async fn delete_by_user(&self, user_id: Uuid) -> Result<Vec<RefreshToken>>;
}
