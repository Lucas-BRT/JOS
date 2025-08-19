use crate::Error;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("username already taken")]
    UsernameAlreadyTaken,
    #[error("email already taken")]
    EmailAlreadyTaken,
    #[error("game system name already taken: {0}")]
    GameSystemNameAlreadyTaken(String),
    #[error("user already has intent for this session")]
    UserSessionIntentAlreadyExists,
    #[error("foreign key violation: table {table}, field {field}")]
    ForeignKeyViolation { table: String, field: String },
    #[error("unknown constraint violation: {0}")]
    UnknownConstraint(String),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user not found")]
    UserNotFound,
    #[error("table not found")]
    TableNotFound,
    #[error("table request not found")]
    TableRequestNotFound,
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        Error::Repository(err)
    }
}
