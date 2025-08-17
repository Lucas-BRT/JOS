use super::password::error::PasswordDomainError;
use super::table::error::TableDomainError;
use super::user::error::UserDomainError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Table error: {0}")]
    Table(TableDomainError),
    #[error("User error: {0}")]
    User(UserDomainError),
    #[error("Password error: {0}")]
    Password(PasswordDomainError),
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
