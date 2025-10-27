use thiserror::Error;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Debug, Error, Clone)]
pub enum UserDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("User not found: {0}")]
    UserNotFound(Uuid),
    #[error("Username already exists: {0}")]
    UsernameAlreadyExists(String),
    #[error("Email already exists: {0}")]
    EmailAlreadyExists(String),
}

#[derive(Debug, Error, Clone)]
pub enum TableDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Table not found: {0}")]
    TableNotFound(Uuid),
    #[error("User {user_id} is not the game master of table {table_id}")]
    NotGameMaster { user_id: Uuid, table_id: Uuid },
    #[error("Table is full")]
    TableFull,
    #[error("Table status does not allow this operation: {0}")]
    InvalidTableStatus(String),
}

#[derive(Debug, Error, Clone)]
pub enum SessionDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Session not found: {0}")]
    SessionNotFound(Uuid),
    #[error("Session is not accepting intents")]
    NotAcceptingIntents,
    #[error("Invalid session status transition: from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },
    #[error("Session is already in progress")]
    SessionInProgress,
    #[error("Session is completed")]
    SessionCompleted,
}

#[derive(Debug, Error, Clone)]
pub enum SessionIntentDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Session intent not found: {0}")]
    SessionIntentNotFound(Uuid),
    #[error("User already has intent for this session")]
    IntentAlreadyExists,
    #[error("Cannot change intent status: session has already started")]
    SessionAlreadyStarted,
    #[error("Invalid intent status transition: from {from} to {to}")]
    InvalidIntentStatusTransition { from: String, to: String },
}

#[derive(Debug, Error, Clone)]
pub enum TableRequestDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Table request not found: {0}")]
    TableRequestNotFound(Uuid),
    #[error("User already has a pending request for this table")]
    RequestAlreadyExists,
    #[error("Cannot approve request: table is full")]
    TableFull,
    #[error("Request has already been processed")]
    RequestAlreadyProcessed,
    #[error("Only the game master can approve/reject requests")]
    NotAuthorized,
}

#[derive(Debug, Error, Clone)]
pub enum GameSystemDomainError {
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Game system not found: {0}")]
    GameSystemNotFound(Uuid),
    #[error("Game system name already exists: {0}")]
    GameSystemNameAlreadyExists(String),
}

#[derive(Debug, Error, Clone)]
pub enum DomainError {
    #[error("User error: {0}")]
    User(#[from] UserDomainError),
    #[error("Table error: {0}")]
    Table(#[from] TableDomainError),
    #[error("Session error: {0}")]
    Session(#[from] SessionDomainError),
    #[error("Session intent error: {0}")]
    SessionIntent(#[from] SessionIntentDomainError),
    #[error("Table request error: {0}")]
    TableRequest(#[from] TableRequestDomainError),
    #[error("Game system error: {0}")]
    GameSystem(#[from] GameSystemDomainError),
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}
