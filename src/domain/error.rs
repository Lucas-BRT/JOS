use super::table::error::TableError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Table error: {0}")]
    Table(TableError),
}
