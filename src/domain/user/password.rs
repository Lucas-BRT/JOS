use crate::{domain::utils::type_wraper::TypeWrapped, error::UserValidationError};
use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{self, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RawPassword(String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct HashPassword(String);

impl TypeWrapped for RawPassword {
    type Raw = String;
    type Error = UserValidationError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let raw = raw.trim().to_string();

        if raw.is_empty() {
            return Err(UserValidationError::Password(
                "Password cannot be empty".to_string(),
            ));
        }

        if raw.len() < MIN_PASSWORD_LENGTH {
            return Err(UserValidationError::Password(format!(
                "Password must be at least {} characters long",
                MIN_PASSWORD_LENGTH
            )));
        }

        if !raw.chars().any(|c| c.is_uppercase()) {
            return Err(UserValidationError::Password(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        if !raw.chars().any(|c| c.is_lowercase()) {
            return Err(UserValidationError::Password(
                "Password must contain at least one lowercase letter".to_string(),
            ));
        }

        if !raw.chars().any(|c| c.is_digit(10)) {
            return Err(UserValidationError::Password(
                "Password must contain at least one digit".to_string(),
            ));
        }

        if !raw.chars().any(|c| c.is_ascii_punctuation()) {
            return Err(UserValidationError::Password(
                "Password must contain at least one punctuation character".to_string(),
            ));
        }

        Ok(RawPassword(raw))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}

impl RawPassword {
    pub fn hash(&self) -> Result<HashPassword, UserValidationError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng::default());

        let hashed =
            password_hash::PasswordHash::generate(argon2, self.0.as_bytes(), salt.as_salt())
                .map_err(|e| UserValidationError::Password(e.to_string()))?;

        Ok(HashPassword(hashed.to_string()))
    }
}

impl TypeWrapped for HashPassword {
    type Raw = String;
    type Error = UserValidationError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        Ok(HashPassword(raw))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}

impl HashPassword {
    pub fn verify(&self, raw_password: &RawPassword) -> Result<(), UserValidationError> {
        let argon2 = Argon2::default();
        let parsed_hash = password_hash::PasswordHash::new(&self.0)
            .map_err(|e| UserValidationError::Password(format!("Invalid hash: {}", e)))?;

        argon2
            .verify_password(raw_password.0.as_bytes(), &parsed_hash)
            .map_err(|e| UserValidationError::Password(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::UserValidationError;

    #[test]
    fn test_valid_password_parsing() {
        let password = "Valid123!";
        let parsed = RawPassword::parse(password.to_string());
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().raw(), password);
    }

    #[test]
    fn test_empty_password() {
        let err = RawPassword::parse("".to_string()).unwrap_err();
        assert!(
            matches!(err, UserValidationError::Password(msg) if msg.contains("cannot be empty"))
        );
    }

    #[test]
    fn test_short_password() {
        let err = RawPassword::parse("A1!a".to_string()).unwrap_err();
        assert!(matches!(err, UserValidationError::Password(msg) if msg.contains("at least")));
    }

    #[test]
    fn test_missing_uppercase() {
        let err = RawPassword::parse("valid123!".to_string()).unwrap_err();
        assert!(matches!(err, UserValidationError::Password(msg) if msg.contains("uppercase")));
    }

    #[test]
    fn test_missing_lowercase() {
        let err = RawPassword::parse("INVALID123!".to_string()).unwrap_err();
        assert!(matches!(err, UserValidationError::Password(msg) if msg.contains("lowercase")));
    }

    #[test]
    fn test_missing_digit() {
        let err = RawPassword::parse("Invalid!".to_string()).unwrap_err();
        assert!(matches!(err, UserValidationError::Password(msg) if msg.contains("digit")));
    }

    #[test]
    fn test_missing_punctuation() {
        let err = RawPassword::parse("Invalid123".to_string()).unwrap_err();
        assert!(matches!(err, UserValidationError::Password(msg) if msg.contains("punctuation")));
    }

    #[test]
    fn test_password_hash_and_verify_success() {
        let raw = RawPassword::parse("Valid123!".to_string()).unwrap();
        let hashed = raw.hash().unwrap();
        let result = hashed.verify(&raw);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_verify_fail_with_wrong_password() {
        let raw = RawPassword::parse("Valid123!".to_string()).unwrap();
        let wrong = RawPassword::parse("Wrong123!".to_string()).unwrap();
        let hashed = raw.hash().unwrap();
        let result = hashed.verify(&wrong);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_hashed_password_raw_roundtrip() {
        let raw = RawPassword::parse("Valid123!".to_string()).unwrap();
        let hashed = raw.hash().unwrap();
        let parsed = HashPassword::parse(hashed.raw()).unwrap();
        assert_eq!(hashed, parsed);
    }
}
