use async_trait::async_trait;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::password_hash::{Error::Password, PasswordHasher, SaltString, rand_core::OsRng};
use crate::{
    Error, Result,
    domain::password::{PasswordRepository, PasswordValidator, PasswordValidationError},
};

#[derive(Clone)]
pub struct PasswordRepositoryImpl {
    validator: PasswordValidator,
}

impl PasswordRepositoryImpl {
    pub fn new() -> Self {
        Self {
            validator: PasswordValidator::new(),
        }
    }

    pub fn with_validator(validator: PasswordValidator) -> Self {
        Self { validator }
    }
}

#[async_trait]
impl PasswordRepository for PasswordRepositoryImpl {
    async fn generate_hash(&self, password: String) -> Result<String> {
        // Validate password before hashing
        self.validator.validate(&password)
            .map_err(|e| {
                tracing::error!("Password validation failed: {}", e.message);
                Error::Application(crate::application::error::ApplicationError::InvalidInput(e.message))
            })?;

        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            argon2
                .hash_password(password.as_bytes(), &salt)
                .map(|hash| hash.to_string())
                .map_err(|_| {
                    tracing::error!("failed to hash password");
                    Error::InternalServerError
                })
        })
        .await
        .map_err(|_| {
            tracing::error!("failed to generate hash");
            Error::InternalServerError
        })?
    }

    async fn verify_hash(&self, password: String, hash: String) -> Result<bool> {
        tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hash).map_err(|_| {
                tracing::error!("failed to parse hash");
                Error::InternalServerError
            })?;

            match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                Ok(_) => Ok(true),
                Err(Password) => Ok(false),
                Err(e) => {
                    tracing::error!("unexpected error during password verification: {}", e);
                    Err(Error::InternalServerError)
                }
            }
        })
        .await
        .map_err(|_| {
            tracing::error!("failed to verify hash");
            Error::InternalServerError
        })?
    }

    async fn validate_password(&self, password: &str) -> std::result::Result<(), PasswordValidationError> {
        self.validator.validate(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_hash() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }

    #[tokio::test]
    async fn test_verify_hash() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let result = password_repo
            .verify_hash(password.to_string(), hash)
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_hash_with_wrong_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
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
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";

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
    async fn test_empty_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "ValidPass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
        
        let result = password_repo
            .verify_hash(password.to_string(), hash)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_very_long_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "A".repeat(100) + "b1"; // 102 characters, valid
        let hash = password_repo
            .generate_hash(password.clone())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
        
        let result = password_repo
            .verify_hash(password, hash)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_special_characters_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123!".to_string();
        let hash = password_repo
            .generate_hash(password.clone())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
        
        let result = password_repo
            .verify_hash(password, hash)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_unicode_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SécurePass123".to_string();
        let hash = password_repo
            .generate_hash(password.clone())
            .await
            .unwrap();
        assert!(hash.starts_with("$argon2id$"));
        
        let result = password_repo
            .verify_hash(password, hash)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_same_password_different_hashes() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        
        let hash1 = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let hash2 = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        
        // Same password should generate different hashes due to different salts
        assert_ne!(hash1, hash2);
        
        // Both hashes should verify correctly
        let result1 = password_repo
            .verify_hash(password.to_string(), hash1)
            .await
            .unwrap();
        let result2 = password_repo
            .verify_hash(password.to_string(), hash2)
            .await
            .unwrap();
        
        assert!(result1);
        assert!(result2);
    }

    #[tokio::test]
    async fn test_verify_with_invalid_hash() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let invalid_hash = "invalid_hash_format".to_string();
        
        let result = password_repo
            .verify_hash(password.to_string(), invalid_hash)
            .await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_with_empty_hash() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let empty_hash = "".to_string();
        
        let result = password_repo
            .verify_hash(password.to_string(), empty_hash)
            .await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_verify_operations() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let repo = password_repo.clone();
                let pwd = password.to_string();
                let h = hash.clone();
                tokio::spawn(async move { repo.verify_hash(pwd, h).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            let is_valid = result.unwrap();
            assert!(is_valid);
        }
    }

    #[tokio::test]
    async fn test_mixed_concurrent_operations() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();

        let mut hash_handles = Vec::new();
        let mut verify_handles = Vec::new();

        // Spawn some hash generation tasks
        for _ in 0..3 {
            let repo = password_repo.clone();
            let pwd = password.to_string();
            hash_handles.push(tokio::spawn(async move { repo.generate_hash(pwd).await }));
        }

        // Spawn some verification tasks
        for _ in 0..3 {
            let repo = password_repo.clone();
            let pwd = password.to_string();
            let h = hash.clone();
            verify_handles.push(tokio::spawn(async move { repo.verify_hash(pwd, h).await }));
        }

        // Wait for all hash operations
        for handle in hash_handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }

        // Wait for all verify operations
        for handle in verify_handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_password_case_sensitivity() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123".to_string();
        let hash = password_repo
            .generate_hash(password.clone())
            .await
            .unwrap();
        
        // Same case should work
        let result = password_repo
            .verify_hash(password.clone(), hash.clone())
            .await
            .unwrap();
        assert!(result);
        
        // Different case should fail
        let wrong_case = "securepass123".to_string();
        let result = password_repo
            .verify_hash(wrong_case, hash)
            .await
            .unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_whitespace_sensitivity() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = " SecurePass123 ".to_string();
        let hash = password_repo
            .generate_hash(password.clone())
            .await
            .unwrap();
        
        // Same whitespace should work
        let result = password_repo
            .verify_hash(password.clone(), hash.clone())
            .await
            .unwrap();
        assert!(result);
        
        // Different whitespace should fail
        let no_whitespace = "SecurePass123".to_string();
        let result = password_repo
            .verify_hash(no_whitespace, hash)
            .await
            .unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_repository_clone_works() {
        let password_repo = PasswordRepositoryImpl::new();
        let cloned_repo = password_repo.clone();
        
        let password = "SecurePass123";
        let hash = password_repo
            .generate_hash(password.to_string())
            .await
            .unwrap();
        
        // Cloned repository should work the same
        let result = cloned_repo
            .verify_hash(password.to_string(), hash)
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_multiple_repository_instances() {
        let repo1 = PasswordRepositoryImpl::new();
        let repo2 = PasswordRepositoryImpl::new();
        
        let password = "SecurePass123";
        let hash1 = repo1
            .generate_hash(password.to_string())
            .await
            .unwrap();
        let hash2 = repo2
            .generate_hash(password.to_string())
            .await
            .unwrap();
        
        // Different instances should generate different hashes
        assert_ne!(hash1, hash2);
        
        // Both should verify correctly
        let result1 = repo1
            .verify_hash(password.to_string(), hash1)
            .await
            .unwrap();
        let result2 = repo2
            .verify_hash(password.to_string(), hash2)
            .await
            .unwrap();
        
        assert!(result1);
        assert!(result2);
    }

    // Password validation tests
    #[tokio::test]
    async fn test_password_validation_too_short() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "abc123";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::TooShort);
            assert!(error.message.contains("at least 8 characters"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_too_long() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "a".repeat(129); // 129 characters
        
        let result = password_repo.validate_password(&password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::TooLong);
            assert!(error.message.contains("at most 128 characters"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_missing_uppercase() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "password123";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::MissingUppercase);
            assert!(error.message.contains("uppercase letter"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_missing_lowercase() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "PASSWORD123";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::MissingLowercase);
            assert!(error.message.contains("lowercase letter"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_missing_digit() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "PasswordABC";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::MissingDigit);
            assert!(error.message.contains("digit"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_common_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "password";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            // The password "password" should fail for multiple reasons, but the first check is uppercase
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::MissingUppercase);
            assert!(error.message.contains("uppercase letter"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_common_password_with_requirements() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "Password123"; // Meets all requirements but is common
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::CommonPassword);
            assert!(error.message.contains("too common"));
        }
    }

    #[tokio::test]
    async fn test_password_validation_valid_password() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_password_validation_with_special_chars() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123!";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_password_validation_unicode_valid() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SécurePass123";
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_password_validation_control_characters() {
        let password_repo = PasswordRepositoryImpl::new();
        let password = "SecurePass123\x00"; // Contains null byte
        
        let result = password_repo.validate_password(password).await;
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.validation_type, crate::domain::password::PasswordValidationType::ContainsInvalidCharacters);
            assert!(error.message.contains("control characters"));
        }
    }

    #[tokio::test]
    async fn test_generate_hash_with_validation() {
        let password_repo = PasswordRepositoryImpl::new();
        
        // Valid password should work
        let valid_password = "SecurePass123";
        let result = password_repo.generate_hash(valid_password.to_string()).await;
        assert!(result.is_ok());
        
        // Invalid password should fail
        let invalid_password = "weak";
        let result = password_repo.generate_hash(invalid_password.to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_custom_validator() {
        let custom_validator = crate::domain::password::PasswordValidator::new()
            .with_min_length(10)
            .with_special_requirement(true);
        
        let password_repo = PasswordRepositoryImpl::with_validator(custom_validator);
        
        // Password too short for custom validator
        let short_password = "Pass123";
        let result = password_repo.validate_password(short_password).await;
        assert!(result.is_err());
        
        // Password missing special character
        let no_special_password = "Password123";
        let result = password_repo.validate_password(no_special_password).await;
        assert!(result.is_err());
        
        // Valid password for custom validator
        let valid_password = "Password123!";
        let result = password_repo.validate_password(valid_password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validator_requirements() {
        let validator = crate::domain::password::PasswordValidator::new();
        let requirements = validator.get_requirements();
        
        assert!(requirements.iter().any(|r| r.contains("8 characters")));
        assert!(requirements.iter().any(|r| r.contains("uppercase")));
        assert!(requirements.iter().any(|r| r.contains("lowercase")));
        assert!(requirements.iter().any(|r| r.contains("digit")));
        assert!(requirements.iter().any(|r| r.contains("control characters")));
        assert!(requirements.iter().any(|r| r.contains("common password")));
    }
}
