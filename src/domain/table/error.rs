use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TableDomainError {
    #[error("Failed to parse database data: {0}")]
    FailedToParseDbData(String),
}
