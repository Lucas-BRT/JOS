use crate::domain::user::error::{PhoneNumberValidationError, UserDomainError};
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumberVo(String);

impl TypeWrapped for PhoneNumberVo {
    type Raw = String;
    type Error = UserDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed_raw = raw.trim();

        if !trimmed_raw.starts_with("+55") {
            return Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid,
            ));
        }

        let normalized_number = trimmed_raw.replace(&[' ', '-', '(', ')'][..], "");

        if !normalized_number[1..].chars().all(|c| c.is_ascii_digit()) {
            return Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid,
            ));
        }

        if normalized_number.len() < 13 {
            return Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooShort,
            ));
        }

        if normalized_number.len() > 14 {
            return Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooLong,
            ));
        }

        Ok(Self(normalized_number))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_brazilian_phone_numbers() {
        let inputs = vec![
            "+55 11 91234-5678",
            "+55(11)91234-5678",
            "+55 11 1234 5678",
            "+55-11-91234-5678",
            "+5511912345678",
        ];

        for input in inputs {
            let result = PhoneNumberVo::parse(input.to_string());
            assert!(result.is_ok(), "Failed to parse valid number: {}", input);
        }
    }

    #[test]
    fn test_invalid_prefix() {
        let input = "+54 11 91234-5678";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid
            ))
        );
    }

    #[test]
    fn test_invalid_length_too_short() {
        let input = "+55 11 123";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooShort
            ))
        );
    }

    #[test]
    fn test_invalid_length_too_long() {
        let input = "+55 11 91234567890123";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooLong
            ))
        );
    }

    #[test]
    fn test_invalid_characters() {
        let input = "+55 11 91234-56A8";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid
            ))
        );
    }

    #[test]
    fn test_missing_plus_sign() {
        let input = "55 11 91234-5678";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid
            ))
        );
    }

    #[test]
    fn test_only_prefix() {
        let input = "+55";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooShort
            ))
        );
    }

    #[test]
    fn test_prefix_without_number() {
        let input = "+55 11";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::TooShort
            ))
        );
    }

    #[test]
    fn test_extra_whitespace() {
        let input = "   +55 11 91234 5678   ";
        let result = PhoneNumberVo::parse(input.to_string());
        assert!(
            result.is_ok(),
            "Failed to parse valid number with extra whitespace: {}",
            input
        );
    }

    #[test]
    fn test_invalid_with_special_symbols() {
        let input = "+55 11 91234@5678";
        let result = PhoneNumberVo::parse(input.to_string());
        assert_eq!(
            result,
            Err(UserDomainError::PhoneNumber(
                PhoneNumberValidationError::Invalid
            ))
        );
    }
}
