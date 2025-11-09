use crate::entities::RefreshToken;
use async_trait::async_trait;
use shared::Error;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &RefreshToken) -> Result<(), Error>;
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, Error>;
    async fn delete_by_token(&self, token: &str) -> Result<(), Error>;
    async fn delete_by_user(&self, user_id: Uuid) -> Result<(), Error>;
}
