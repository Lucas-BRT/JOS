use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PasswordProvider: Send + Sync {
    async fn validate_password(&self, password: &str) -> Result<()>;
    async fn generate_hash(&self, password: String) -> Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> Result<bool>;
}
