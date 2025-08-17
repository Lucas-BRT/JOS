use crate::{domain::password::error::PasswordDomainError, Error, Result};

const DEFAULT_MIN_LENGTH: usize = 8;
const DEFAULT_MAX_LENGTH: usize = 128;
const DEFAULT_MIN_UPPERCASE: usize = 1;
const DEFAULT_MIN_LOWERCASE: usize = 1;
const DEFAULT_MIN_DIGITS: usize = 1;
const DEFAULT_MIN_SPECIAL_CHARS: usize = 1;

pub trait PasswordValidator: Send + Sync {
    fn validate(&self, password: &str) -> Result<()>;
}


#[derive(Clone, Default)]
pub struct DefaultPasswordValidator;

impl PasswordValidator for DefaultPasswordValidator {
    fn validate(&self, password: &str) -> Result<()> {

        let password_length = password.len();

        if password_length < DEFAULT_MIN_LENGTH {
            return Err(Error::Domain(PasswordDomainError::TooShort.into()));
        }

        if password_length > DEFAULT_MAX_LENGTH {
            return Err(Error::Domain(PasswordDomainError::TooLong.into()));
        }

        if password.chars().filter(|c| c.is_uppercase()).count() < DEFAULT_MIN_UPPERCASE {
            return Err(Error::Domain(PasswordDomainError::MissingUppercase.into()));
        }

        if password.chars().filter(|c| c.is_lowercase()).count() < DEFAULT_MIN_LOWERCASE {
            return Err(Error::Domain(PasswordDomainError::MissingLowercase.into()));
        }

        if password.chars().filter(|c| c.is_ascii_digit()).count() < DEFAULT_MIN_DIGITS {
            return Err(Error::Domain(PasswordDomainError::MissingDigit.into()));
        }

        if password.chars().filter(|c| c.is_ascii_punctuation()).count() < DEFAULT_MIN_SPECIAL_CHARS {
            return Err(Error::Domain(PasswordDomainError::MissingSpecialChar.into()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::IndexedRandom;

    use super::*;


    fn generate_valid_password(n: usize) -> String {
        use rand::{seq::SliceRandom, Rng};


        const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const DIGITS: &[u8] = b"0123456789";
        const SPECIAL: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

        let mut rng = rand::rng();
        let mut password = Vec::with_capacity(n);

        for _ in 0..DEFAULT_MIN_UPPERCASE {
            password.push(*UPPERCASE.choose(&mut rng).unwrap());
        }
        for _ in 0..DEFAULT_MIN_LOWERCASE {
            password.push(*LOWERCASE.choose(&mut rng).unwrap());
        }
        for _ in 0..DEFAULT_MIN_DIGITS {
            password.push(*DIGITS.choose(&mut rng).unwrap());
        }
        for _ in 0..DEFAULT_MIN_SPECIAL_CHARS {
            password.push(*SPECIAL.choose(&mut rng).unwrap());
        }

        let all: Vec<u8> = [UPPERCASE, LOWERCASE, DIGITS, SPECIAL].concat();
        for _ in password.len()..n {
            password.push(*all.choose(&mut rng).unwrap());
        }

        password.shuffle(&mut rng);

        String::from_utf8(password).unwrap()
    }

    #[tokio::test]
    async fn test_validate_password_too_short() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("1234567");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_password_too_long() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("12345678901234567890123456789012345678901234567890123456789012345678901234567890");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_password_missing_uppercase() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("password123");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_password_missing_lowercase() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("PASSWORD123");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_password_missing_digit() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("PasswordABC");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_password_valid() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("Password123!");
        assert!(result.is_ok());
    }


    #[tokio::test]
    async fn test_validate_password_missing_special_char() {
        let validator = DefaultPasswordValidator;
        let result = validator.validate("Password123");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_valid_passwords() {

        let validator = DefaultPasswordValidator;
        let mut passwords = Vec::new();

        for _ in 0..100 {
            let password = generate_valid_password(10);
            passwords.push(password);
        }

        for password in passwords {

            let result = validator.validate(&password);
            assert!(result.is_ok());
        }

    }

}


