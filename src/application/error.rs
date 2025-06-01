use crate::core::error::AppError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ApplicationError {
    #[error("Data error: {0}")]
    DataError(String),
    #[error("not found: {0}")]
    NotFound(String),
}

impl From<ApplicationError> for AppError {
    fn from(err: ApplicationError) -> Self {
        AppError::Application(err)
    }
}
