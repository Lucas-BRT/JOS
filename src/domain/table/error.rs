use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TableDomainError {
    #[error("Table not found")]
    NotFound,
    #[error("Table already exists")]
    AlreadyExists,
    #[error("Invalid Description: {0}")]
    InvalidDescription(DescriptionValidationError),
    #[error("Invalid Title: {0}")]
    InvalidTitle(TitleValidationError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DescriptionValidationError {
    #[error("Description is too short")]
    TooShort,
    #[error("Description is too long")]
    TooLong,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TitleValidationError {
    #[error("Title is too short")]
    TooShort,
    #[error("Title is too long")]
    TooLong,
}
