use thiserror::Error;

use crate::core::error::AppError;

use super::table::error::TableDomainError;
use super::user::error::UserDomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Table error: {0}")]
    Table(TableDomainError),
    #[error("User error: {0}")]
    User(UserDomainError),
}

impl From<UserDomainError> for DomainError {
    fn from(error: UserDomainError) -> Self {
        DomainError::User(error)
    }
}

impl From<TableDomainError> for DomainError {
    fn from(error: TableDomainError) -> Self {
        DomainError::Table(error)
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        AppError::Domain(error)
    }
}
