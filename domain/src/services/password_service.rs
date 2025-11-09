use async_trait::async_trait;
use shared::Error;

#[async_trait]
pub trait IPasswordService: Send + Sync {
    async fn generate_hash(&self, password: &str) -> Result<String, Error>;
    async fn verify_hash(&self, password: &str, hash: &str) -> Result<bool, Error>;
}
