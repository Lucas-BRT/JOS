use crate::Error;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplicationError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

impl From<ApplicationError> for Error {
    fn from(err: ApplicationError) -> Self {
        Error::Application(err)
    }
}
