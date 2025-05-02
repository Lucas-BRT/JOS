#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    ApplicationSetupError(String),
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
}
