#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    ApplicationSetup(String),
    #[error("Validation error: {0}")]
    Validation(ValidationError),
}

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("")]
    User(UserValidationError),
}

#[derive(thiserror::Error, Debug)]
pub enum UserValidationError {
    #[error("failed to parse username: {0}")]
    Username(String),
    #[error("failed to parse display name: {0}")]
    DisplayName(String),
    #[error("failed to parse email: {0}")]
    Email(String),
    #[error("failed to parse password: {0}")]
    Password(String),
}
