use bcrypt::{DEFAULT_COST, hash, verify};
use domain::auth::PasswordProvider;
use shared::Result;
use shared::error::Error;
use validator::ValidationError;

#[derive(Clone)]
pub struct BcryptPasswordProvider;

impl Default for BcryptPasswordProvider {
    fn default() -> Self {
        Self
    }
}

impl BcryptPasswordProvider {
    fn validate_password(&self, password: &str) -> Result<()> {
        let mut errors = validator::ValidationErrors::new();

        if password.len() < 8 {
            errors.add("password", ValidationError::new("min_length"));
        }
        if password.len() > 128 {
            errors.add("password", ValidationError::new("max_length"));
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            errors.add("password", ValidationError::new("uppercase_required"));
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            errors.add("password", ValidationError::new("lowercase_required"));
        }
        if !password.chars().any(|c| c.is_numeric()) {
            errors.add("password", ValidationError::new("numeric_required"));
        }
        if !password.chars().any(|c| c.is_ascii_punctuation()) {
            errors.add("password", ValidationError::new("special_required"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(
                shared::error::ValidationError::ValidationFailed(errors.to_string()),
            ))
        }
    }
}

impl BcryptPasswordProvider {
    async fn hash_password(&self, password: String) -> Result<String> {
        tokio::task::spawn_blocking(move || {
            hash(password, DEFAULT_COST).map_err(|e| {
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
impl PasswordProvider for BcryptPasswordProvider {
    async fn generate_hash(&self, password: String) -> Result<String> {
        self.validate_password(&password)?;
        self.hash_password(password).await
    }

    async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        tokio::task::spawn_blocking(move || {
            verify(password, &hash).map_err(|e| {
                tracing::error!("failed to verify hash: {}", e);
                Error::InternalServerError
            })
        })
        .await
        .map_err(|_| Error::InternalServerError)?
    }

    async fn validate_password(&self, password: &str) -> Result<()> {
        self.validate_password(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_hash() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password.into()).await.unwrap();
        assert!(hash.starts_with("$2b$"));
    }

    #[tokio::test]
    async fn test_verify_hash() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password.into()).await.unwrap();
        let result = password_repo.verify_hash(password.into(), hash).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_hash_with_wrong_password() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password.into()).await.unwrap();
        let result = password_repo
            .verify_hash("WrongPass123".into(), hash)
            .await
            .unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_concurrent_hash_operations() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password.into();
                tokio::spawn(async move { repo.generate_hash(pwd).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            let hash = result.unwrap();
            assert!(hash.starts_with("$2b$"));
        }
    }

    #[tokio::test]
    async fn test_verify_with_invalid_hash() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let invalid_hash = "not-a-valid-hash".into();

        let result = password_repo
            .verify_hash(password.into(), invalid_hash)
            .await;
        assert!(result.is_err(), "hash inválido deveria falhar");
    }

    #[tokio::test]
    async fn test_hashes_are_different_for_same_password() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";

        let hash1 = password_repo.generate_hash(password.into()).await.unwrap();
        let hash2 = password_repo.generate_hash(password.into()).await.unwrap();

        assert_ne!(
            hash1, hash2,
            "hashes não deveriam ser iguais por causa do salt"
        );
    }

    #[tokio::test]
    async fn test_concurrent_verify_operations() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password.into()).await.unwrap();

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password.into();
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
