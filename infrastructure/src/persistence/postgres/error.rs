use shared::error::{ApplicationError, Error, PersistenceError};

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
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
    #[error("user already member of table")]
    UserAlreadyMemberOfTable,
    #[error("foreign key violation: table {table}, field {field}")]
    ForeignKeyViolation { table: String, field: String },
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user not found: {0}")]
    UserNotFound(Uuid),
    #[error("game system not found: {0}")]
    GameSystemNotFound(Uuid),
    #[error("rpg table not found: {0}")]
    RpgTableNotFound(Uuid),
    #[error("table not found: {0}")]
    TableNotFound(Uuid),
    #[error("table request not found: {0}")]
    TableRequestNotFound(Uuid),
    #[error("session not found: {0}")]
    SessionNotFound(Uuid),
    #[error("session intent not found: {0}")]
    SessionIntentNotFound(Uuid),
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
        match err {
            RepositoryError::UserNotFound(id) => Error::Persistence(PersistenceError::NotFound {
                entity: "user",
                id: id.to_string(),
            }),
            RepositoryError::TableNotFound(id) => Error::Persistence(PersistenceError::NotFound {
                entity: "table",
                id: id.to_string(),
            }),
            RepositoryError::GameSystemNotFound(id) => {
                Error::Persistence(PersistenceError::NotFound {
                    entity: "game_system",
                    id: id.to_string(),
                })
            }
            RepositoryError::RpgTableNotFound(id) => {
                Error::Persistence(PersistenceError::NotFound {
                    entity: "rpg_table",
                    id: id.to_string(),
                })
            }
            RepositoryError::TableRequestNotFound(id) => {
                Error::Persistence(PersistenceError::NotFound {
                    entity: "table_request",
                    id: id.to_string(),
                })
            }
            RepositoryError::SessionNotFound(id) => {
                Error::Persistence(PersistenceError::NotFound {
                    entity: "session",
                    id: id.to_string(),
                })
            }
            RepositoryError::SessionIntentNotFound(id) => {
                Error::Persistence(PersistenceError::NotFound {
                    entity: "session_intent",
                    id: id.to_string(),
                })
            }
            RepositoryError::UsernameAlreadyTaken => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: "users_username_key".to_string(),
                })
            }
            RepositoryError::EmailAlreadyTaken => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: "users_email_key".to_string(),
                })
            }
            RepositoryError::GameSystemNameAlreadyTaken => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: "game_systems_name_key".to_string(),
                })
            }
            RepositoryError::UserSessionIntentAlreadyExists => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: "session_intents_user_id_session_id_key".to_string(),
                })
            }
            RepositoryError::UserAlreadyMemberOfTable => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: "table_members_user_id_table_id_key".to_string(),
                })
            }
            RepositoryError::ForeignKeyViolation { table, field } => {
                Error::Persistence(PersistenceError::ConstraintViolation {
                    constraint: format!(
                        "foreign key violation on table '{}' for field '{}'",
                        table, field
                    ),
                })
            }
            RepositoryError::DatabaseError(e) => {
                Error::Persistence(PersistenceError::DatabaseError(e.into()))
            }
            RepositoryError::IoError(e) => {
                Error::Persistence(PersistenceError::ConnectionError(e.into()))
            }
            RepositoryError::DatabaseTimeout => {
                Error::Persistence(PersistenceError::ConnectionError(Box::new(
                    std::io::Error::new(std::io::ErrorKind::TimedOut, "Database timeout"),
                )))
            }
            RepositoryError::UnknownConstraint(constraint) => {
                Error::Persistence(PersistenceError::ConstraintViolation { constraint })
            }
            RepositoryError::NotFound => Error::Persistence(PersistenceError::NotFound {
                entity: "unknown",
                id: "unknown".to_string(),
            }),
            RepositoryError::InvalidInput(message) => {
                Error::Application(ApplicationError::InvalidInput { message })
            }
            RepositoryError::ValidationError(message) => {
                Error::Application(ApplicationError::InvalidInput { message })
            }
        }
    }
}
