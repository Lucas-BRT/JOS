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

    #[test]
    fn test_parse_removes_whitespace() {
        let raw = "  my_password  ".to_string();
        let parsed = PasswordVo::parse(raw).unwrap();
        assert_eq!(parsed.raw(), "my_password");
    }

    #[test]
    fn test_hash_generates_hashed_password() {
        let raw = PasswordVo::parse("password123".to_string()).unwrap();
        let hashed = raw.hash().unwrap();
        assert_ne!(hashed.raw(), "password123"); // hash deve ser diferente do original
    }

    #[test]
    fn test_verify_correct_password() {
        let raw_password = "password123";
        let raw = PasswordVo::parse(raw_password.to_string()).unwrap();
        let hashed = raw.hash().unwrap();

        assert!(hashed.verify(raw_password).is_ok());
    }

    #[test]
    fn test_verify_incorrect_password() {
        let raw_password = "password123";
        let wrong_password = "wrongpass";
        let raw = PasswordVo::parse(raw_password.to_string()).unwrap();
        let hashed = raw.hash().unwrap();

        let result = hashed.verify(wrong_password);
        assert!(matches!(result, Err(PasswordDomainError::PasswordMismatch)));
    }

    #[test]
    fn test_raw_returns_inner_password() {
        let raw = "some_password".to_string();
        let parsed = PasswordVo::parse(raw.clone()).unwrap();
        assert_eq!(parsed.raw(), "some_password");
    }
}
