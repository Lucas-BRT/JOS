use shared::error::{ApplicationError as SharedApplicationError, Error};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplicationError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

impl From<ApplicationError> for Error {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::InvalidCredentials => {
                Error::Application(SharedApplicationError::InvalidCredentials)
            }
            ApplicationError::InvalidInput(msg) => {
                Error::Application(SharedApplicationError::InvalidInput(msg))
            }
        }
    }
}
