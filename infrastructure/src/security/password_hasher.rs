use bcrypt::{DEFAULT_COST, hash, verify};
use domain::repositories::PasswordProvider;
use shared::error::{Error, InfrastructureError};
use tokio::task::spawn_blocking;

#[derive(Clone)]
pub struct BcryptPasswordProvider;

impl Default for BcryptPasswordProvider {
    fn default() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PasswordProvider for BcryptPasswordProvider {
    async fn generate_hash(&self, password: &str) -> Result<String, Error> {
        // clone the password to get ownership, so if the password is used elsewhere,
        // it won't be borrowed and the thread can continue executing
        let password = password.to_string();

        let task = spawn_blocking(move || {
            hash(password.clone(), DEFAULT_COST).map_err(|bcript_error| {
                InfrastructureError::HashingFailed(bcript_error.to_string())
            })
        })
        .await;

        let task_result = task
            .map_err(|thread_error| {
                Error::Infrastructure(InfrastructureError::HashingFailed(thread_error.to_string()))
            })?
            .map_err(|hashing_error| {
                Error::Infrastructure(InfrastructureError::HashingFailed(
                    hashing_error.to_string(),
                ))
            })?;

        Ok(task_result)
    }

    async fn verify_hash(&self, password: &str, hash: &str) -> Result<bool, Error> {
        // clone the password to avoid borrowing issues
        let password = password.to_string();
        let hash = hash.to_string();

        let task = spawn_blocking(move || verify(&password, &hash)).await;

        let task_result = task
            .map_err(|thread_error| {
                Error::Infrastructure(InfrastructureError::HashingFailed(thread_error.to_string()))
            })?
            .map_err(|hashing_error| {
                Error::Infrastructure(InfrastructureError::HashingFailed(
                    hashing_error.to_string(),
                ))
            })?;

        Ok(task_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_hash() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password).await.unwrap();
        assert!(hash.starts_with("$2b$"));
    }

    #[tokio::test]
    async fn test_verify_hash() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password).await.unwrap();
        let result = password_repo.verify_hash(password, &hash).await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_hash_with_wrong_password() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password).await.unwrap();
        let result = password_repo
            .verify_hash("WrongPass123", &hash)
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
                let pwd = password;
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
        let invalid_hash = "not-a-valid-hash";

        let result = password_repo.verify_hash(password, invalid_hash).await;
        assert!(result.is_err(), "hash inválido deveria falhar");
    }

    #[tokio::test]
    async fn test_hashes_are_different_for_same_password() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";

        let hash1 = password_repo.generate_hash(password).await.unwrap();
        let hash2 = password_repo.generate_hash(password).await.unwrap();

        assert_ne!(
            hash1, hash2,
            "hashes não deveriam ser iguais por causa do salt"
        );
    }

    #[tokio::test]
    async fn test_concurrent_verify_operations() {
        let password_repo = BcryptPasswordProvider;
        let password = "SecurePass123!";
        let hash = password_repo.generate_hash(password).await.unwrap();

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password;
                let h = hash.clone();
                tokio::spawn(async move { repo.verify_hash(pwd, &h).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap().unwrap();
            assert!(result);
        }
    }
}
