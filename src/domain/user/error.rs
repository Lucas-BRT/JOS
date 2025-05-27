use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserDomainError {
    #[error("Username error: {0}")]
    Username(#[from] UsernameDomainError),
    #[error("Password error: {0}")]
    Password(#[from] PasswordDomainError),
    #[error("Display name error: {0}")]
    DisplayName(#[from] DisplayNameDomainError),
    #[error("Phone number error: {0}")]
    PhoneNumber(#[from] PhoneNumberValidationError),
    #[error("Email error: {0}")]
    Email(#[from] EmailDomainError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PasswordDomainError {
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Hashing error: {0}")]
    HashingError(String),
    #[error("Password mismatch")]
    PasswordMismatch,
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DisplayNameDomainError {
    #[error("Display name is too short")]
    TooShort,
    #[error("Display name is too long")]
    TooLong,
    #[error("Display name cannot be empty")]
    Empty,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PhoneNumberValidationError {
    #[error("Invalid phone number format")]
    Invalid,
    #[error("Phone number is too short")]
    TooShort,
    #[error("Phone number is too long")]
    TooLong,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UsernameDomainError {
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("Username is too short")]
    TooShort,
    #[error("Username is too long")]
    TooLong,
    #[error("Username cannot be empty")]
    Empty,
    #[error("Username contains invalid characters")]
    InvalidCharacters,
    #[error("Username cannot start with underscore")]
    StartsWithUnderscore,
    #[error("Username cannot end with underscore")]
    EndsWithUnderscore,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EmailDomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Email is too short")]
    TooShort,
    #[error("Email is too long")]
    TooLong,
    #[error("Email cannot be empty")]
    Empty,
}
