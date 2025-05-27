use super::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::domain::user::error::PasswordDomainError;
use argon2::{
    Argon2,
    password_hash::{self, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hashed;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Raw;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordVo<State = Raw> {
    password: String,
    state: PhantomData<State>,
}

impl PasswordVo<Raw> {
    pub fn parse(raw: String) -> Result<Self, PasswordDomainError> {
        let raw = raw.trim();

        Ok(PasswordVo {
            password: raw.to_string(),
            state: PhantomData::<Raw>,
        })
    }

    pub fn hash(self) -> Result<PasswordVo<Hashed>, PasswordDomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(self.password.as_bytes(), &salt)
            .map_err(|err| PasswordDomainError::HashingError(err.to_string()))?
            .to_string();

        Ok(PasswordVo {
            password: hashed_password,
            state: PhantomData::<Hashed>,
        })
    }
}

impl PasswordVo<Hashed> {
    pub fn verify(&self, raw_password: &str) -> Result<(), PasswordDomainError> {
        let argon2 = Argon2::default();

        let parsed_hash = password_hash::PasswordHash::new(&self.password)
            .map_err(|err| PasswordDomainError::HashingError(err.to_string()))?;

        argon2
            .verify_password(raw_password.as_bytes(), &parsed_hash)
            .map_err(|_| PasswordDomainError::PasswordMismatch)?;

        Ok(())
    }
}

impl<State> PasswordVo<State> {
    pub fn raw(&self) -> String {
        self.password.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::error::PasswordDomainError;

    const VALID_PASSWORD: &str = "Valid1!";

    #[test]
    fn test_parse_empty_password() {
        let result = PasswordVo::parse("".to_string());
        assert!(matches!(result, Err(PasswordDomainError::Empty)));
    }

    #[test]
    fn test_parse_too_short_password() {
        let result = PasswordVo::parse("A1!".to_string());
        assert!(matches!(result, Err(PasswordDomainError::TooShort)));
    }

    #[test]
    fn test_parse_too_long_password() {
        let long_password = "A1!".repeat(MAX_PASSWORD_LENGTH + 1);
        let result = PasswordVo::parse(long_password);
        assert!(matches!(result, Err(PasswordDomainError::TooLong)));
    }

    #[test]
    fn test_parse_missing_uppercase() {
        let result = PasswordVo::parse("valid1!".to_string());
        assert!(matches!(result, Err(PasswordDomainError::MissingUppercase)));
    }

    #[test]
    fn test_parse_missing_lowercase() {
        let result = PasswordVo::parse("VALID1!".to_string());
        assert!(matches!(result, Err(PasswordDomainError::MissingLowercase)));
    }

    #[test]
    fn test_parse_missing_digit() {
        let result = PasswordVo::parse("Valid!".to_string());
        assert!(matches!(result, Err(PasswordDomainError::MissingDigit)));
    }

    #[test]
    fn test_parse_missing_punctuation() {
        let result = PasswordVo::parse("Valid1".to_string());
        assert!(matches!(
            result,
            Err(PasswordDomainError::MissingPunctuation)
        ));
    }

    #[test]
    fn test_parse_valid_password() {
        let result = PasswordVo::parse(VALID_PASSWORD.to_string());
        assert!(result.is_ok());

        let password_vo = result.unwrap();
        assert_eq!(password_vo.raw(), VALID_PASSWORD);
    }

    #[test]
    fn test_hash_and_verify_success() {
        let raw_password = VALID_PASSWORD.to_string();
        let parsed = PasswordVo::parse(raw_password.clone()).unwrap();

        let hashed = parsed.hash().unwrap();
        hashed.verify(&raw_password).unwrap();
    }

    #[test]
    fn test_raw_method() {
        let parsed = PasswordVo::parse(VALID_PASSWORD.to_string()).unwrap();
        assert_eq!(parsed.raw(), VALID_PASSWORD);
    }
}
