#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("username already taken: {0}")]
    UsernameAlreadyTaken(String),
    #[error("email already taken: {0}")]
    EmailAlreadyTaken(String),
    #[error("not found {0}")]
    NotFound(String),
    #[error("unique violation {0}")]
    UniqueViolation(String),
    #[error("foreign key violation {0}")]
    ForeignKeyViolation(String),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("connection error {0}")]
    ConnectionError(String),
    #[error("unexpected error {0}")]
    Unexpected(String),
}
