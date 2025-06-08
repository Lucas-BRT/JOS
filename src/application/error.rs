use axum::response::IntoResponse;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplicationError {
    #[error("Data error: {0}")]
    DataError(String),
    #[error("not found: {0}")]
    NotFound(String),
}

impl From<ApplicationError> for Error {
    fn from(err: ApplicationError) -> Self {
        Error::Application(err)
    }
}
