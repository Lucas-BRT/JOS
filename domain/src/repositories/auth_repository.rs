use crate::auth::Claims;
use async_trait::async_trait;
use shared::Error;
use uuid::Uuid;

#[async_trait]
pub trait PasswordProvider: Send + Sync {
    async fn generate_hash(&self, password: &str) -> Result<String, Error>;
    async fn verify_hash(&self, password: &str, hash: &str) -> Result<bool, Error>;
}

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn generate_token(&self, user_id: Uuid) -> Result<String, Error>;
    async fn decode_token(&self, token: &str) -> Result<Claims, Error>;
}
