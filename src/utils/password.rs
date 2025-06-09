use crate::Error;
use crate::Result;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub async fn generate_hash(password: String) -> Result<String> {
    let hash = tokio::task::spawn_blocking(move || {
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
    })??;

    Ok(hash)
}

pub async fn verify_hash(password: String, hash: String) -> Result<()> {
    let result = tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash).map_err(|_| {
            tracing::error!("failed to parse hash");
            return Error::InternalServerError;
        })?;

        let argon2 = Argon2::default();

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| {
                tracing::error!("failed to verify password");
                Error::InternalServerError
            })
    })
    .await
    .map_err(|_| {
        tracing::error!("failed to verify hash");
        Error::InternalServerError
    })??;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_hash() {
        let password = "password123";
        let hash = generate_hash(password.to_string()).await.unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }

    #[tokio::test]
    async fn test_verify_hash() {
        let password = "password123";
        let hash = generate_hash(password.to_string()).await.unwrap();
        let result = verify_hash(password.to_string(), hash).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_hash_with_wrong_password() {
        let password = "password123";
        let hash = generate_hash(password.to_string()).await.unwrap();
        let result = verify_hash("wrong_password".to_string(), hash).await;
        assert!(result.is_err());
    }
}
