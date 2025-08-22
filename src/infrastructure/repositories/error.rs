use crate::Error;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Io error: {0}")]
    IoError(String),
    #[error("database timeout")]
    DatabaseTimeout,
    #[error("username already taken")]
    UsernameAlreadyTaken,
    #[error("email already taken")]
    EmailAlreadyTaken,
    #[error("game system name already taken")]
    GameSystemNameAlreadyTaken,
    #[error("user already has intent for this session")]
    UserSessionIntentAlreadyExists,
    #[error("foreign key violation: table {table}, field {field}")]
    ForeignKeyViolation { table: String, field: String },
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user not found: {0}")]
    UserNotFound(String),
    #[error("game system not found: {0}")]
    GameSystemNotFound(String),
    #[error("rpg table not found: {0}")]
    RpgTableNotFound(String),
    #[error("table not found")]
    TableNotFound,
    #[error("table request not found")]
    TableRequestNotFound,
    #[error("session not found")]
    SessionNotFound,
    #[error("session intent not found: {0}")]
    SessionIntentNotFound(String),
    #[error("unknown constraint: {0}")]
    UnknownConstraint(String),
    #[error("not found")]
    NotFound,
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("validation error: {0}")]
    ValidationError(String),
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        Error::Repository(err)
    }
}
