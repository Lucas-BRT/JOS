use thiserror::Error;

use crate::core::error::AppError;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Not found")]
    NotFound,
    #[error("Duplicate")]
    Duplicate,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Data mapping error: {0}")]
    DataMappingError(String),
    #[error("Database error: {0}")]
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::Database(_) => RepositoryError::DatabaseError(err),
            _ => RepositoryError::DatabaseError(err),
        }
    }
}

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        AppError::Repository(err)
    }
}
