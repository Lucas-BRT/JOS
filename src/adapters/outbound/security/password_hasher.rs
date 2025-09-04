use crate::domain::password::validator::DefaultPasswordValidator;
use crate::{
    Error, Result,
    domain::password::{PasswordProvider, PasswordValidator},
};
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use std::sync::Arc;

#[derive(Clone)]
pub struct Argon2PasswordProvider {
    validator: Arc<dyn PasswordValidator>,
}

impl Default for Argon2PasswordProvider {
    fn default() -> Self {
        Self::new(Arc::new(DefaultPasswordValidator))
    }
}

impl Argon2PasswordProvider {
    pub fn new(validator: Arc<dyn PasswordValidator>) -> Self {
        Self { validator }
    }
}

impl Argon2PasswordProvider {
    async fn hash_password(&self, password: String) -> Result<String> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            argon2
                .hash_password(password.as_bytes(), &salt)
                .map(|hash| hash.to_string())
                .map_err(|e| {
                    tracing::error!("failed to generate hash: {}", e);
                    Error::InternalServerError
                })
        })
        .await
        .map_err(|e| {
            tracing::error!("failed to generate hash: {}", e);
            Error::InternalServerError
        })?
    }
}

#[async_trait::async_trait]
impl PasswordProvider for Argon2PasswordProvider {
    async fn generate_hash(&self, password: String) -> Result<String> {
        self.validator.validate(&password)?;
        self.hash_password(password).await
    }

    async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).map_err(|_| Error::InternalServerError)?;

            Ok(Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok())
        })
        .await
        .map_err(|_| Error::InternalServerError)?
    }

    async fn validate_password(&self, password: &str) -> Result<()> {
        self.validator.validate(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_hash() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }

    #[tokio::test]
    async fn test_verify_hash() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let result = password_repo.verify_hash(password.to_string(), hash).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_hash_with_wrong_password() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let result = password_repo
            .verify_hash("WrongPass123".to_string(), hash)
            .await
            .unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_concurrent_hash_operations() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password.to_string();
                tokio::spawn(async move { repo.generate_hash(pwd).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            let hash = result.unwrap();
            assert!(hash.starts_with("$argon2id$"));
        }
    }

    #[tokio::test]
    async fn test_verify_with_invalid_hash() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";
        let invalid_hash = "not-a-valid-hash".to_string();

        let result = password_repo
            .verify_hash(password.to_string(), invalid_hash)
            .await;
        assert!(result.is_err(), "hash inválido deveria falhar");
    }

    #[tokio::test]
    async fn test_hashes_are_different_for_same_password() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";

        let hash1 = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let hash2 = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();

        assert_ne!(
            hash1, hash2,
            "hashes não deveriam ser iguais por causa do salt"
        );
    }

    #[tokio::test]
    async fn test_concurrent_verify_operations() {
        let password_repo = Argon2PasswordProvider::default();
        let password = "SecurePass123!";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password.to_string();
                let h = hash.clone();
                tokio::spawn(async move { repo.verify_hash(pwd, h).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap().unwrap();
            assert!(result);
        }
    }
}
