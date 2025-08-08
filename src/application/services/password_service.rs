use std::sync::Arc;
use crate::{
    domain::password::{PasswordRepository, PasswordValidationError},
    Result,
};

#[derive(Clone)]
pub struct PasswordService {
    password_repository: Arc<dyn PasswordRepository>,
}

impl PasswordService {
    pub fn new(password_repository: Arc<dyn PasswordRepository>) -> Self {
        Self { password_repository }
    }

    pub async fn generate_hash(&self, password: String) -> Result<String> {
        self.password_repository.generate_hash(password).await
    }

    pub async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        self.password_repository.verify_hash(password, hash).await
    }

    pub async fn validate_password(&self, password: &str) -> std::result::Result<(), PasswordValidationError> {
        self.password_repository.validate_password(password).await
    }

    pub async fn get_requirements(&self) -> Vec<String> {
        // For now, return default requirements. In the future, this could be configurable
        vec![
            "At least 8 characters long".to_string(),
            "At most 128 characters long".to_string(),
            "At least one uppercase letter".to_string(),
            "At least one lowercase letter".to_string(),
            "At least one digit".to_string(),
            "No control characters allowed".to_string(),
            "Cannot be a common password".to_string(),
        ]
    }
}
