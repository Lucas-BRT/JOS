use async_trait::async_trait;
use uuid::Uuid;

use crate::Result;
use crate::domain::entities::RefreshToken;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> Result<()>;
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>>;
    async fn delete_by_token(&self, token: &str) -> Result<()>;
    async fn delete_by_user(&self, user_id: &Uuid) -> Result<()>;
}
