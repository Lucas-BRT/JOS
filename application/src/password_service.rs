use shared::Result;
use domain::auth::PasswordProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct PasswordService {
    password_provider: Arc<dyn PasswordProvider>,
}

impl PasswordService {
    pub fn new(password_provider: Arc<dyn PasswordProvider>) -> Self {
        Self { password_provider }
    }

    pub async fn generate_hash(&self, password: String) -> Result<String> {
        self.password_provider.generate_hash(password).await
    }

    pub async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        self.password_provider.verify_hash(password, hash).await
    }

    pub async fn validate_password(&self, password: &str) -> Result<()> {
        self.password_provider.validate_password(password).await
    }
}
