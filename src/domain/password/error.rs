use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PasswordDomainError {
    #[error("Password is too common")]
    TooCommon,
    #[error("Password is missing uppercase")]
    MissingUppercase,
    #[error("Password is missing lowercase")]
    MissingLowercase,
    #[error("Password is missing number")]
    MissingNumber,
    #[error("Password is missing special character")]
    MissingSpecialChar,
    #[error("Password is missing digit")]
    MissingDigit,
    #[error("Password is missing punctuation")]
    MissingPunctuation,
    #[error("Password is too short")]
    TooShort,
    #[error("Password is too long")]
    TooLong,
    #[error("Password is too weak")]
    TooWeak,
    #[error("Password cannot be empty")]
    Empty,
}

impl From<PasswordDomainError> for DomainError {
    fn from(error: PasswordDomainError) -> Self {
        DomainError::Password(error)
    }
}
