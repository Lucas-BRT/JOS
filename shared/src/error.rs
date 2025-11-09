use log::error;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Domain error: {0}")]
    Domain(DomainError),
    #[error("Infrastructure error: {0}")]
    Infrastructure(InfrastructureError),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DomainError {
    // --- Table Errors ---
    #[error("Table not found")]
    TableNotFound,
    #[error("Title cannot be empty")]
    EmptyTitle,
    #[error("Description cannot be empty")]
    EmptyDescription,
    #[error("Table slots must be greater than zero")]
    ZeroSlots,
    #[error("Only the table's Game Master can perform this action")]
    UserNotTableGameMaster,
    #[error("The table is full")]
    TableIsFull,
    #[error("The user is already a member of this table")]
    UserIsAlreadyMember,
    #[error("The table must be 'Active' to start a session")]
    TableNotActive,
    #[error("The user is not a member of this table")]
    UserNotTableMember,

    // --- TableRequest Errors ---
    #[error("The request is not in a pending state")]
    RequestNotPending,
    #[error("A user cannot submit more than one request to the same table")]
    DuplicateTableRequest,

    // --- Session Errors ---
    #[error("A session cannot be scheduled in the past")]
    SessionScheduledInPast,
    #[error("The session must be in 'Scheduled' state for this action")]
    SessionNotScheduled,

    // --- Session Checkin Errors ---
    #[error("The session checkin must be in 'Pending' state for this action")]
    SessionCheckinNotPending,
    #[error("The session checkin must be in 'Scheduled' state for this action")]
    SessionCheckinNotScheduled,
    #[error("The session checkin must be in 'CheckedIn' state for this action")]
    SessionCheckinNotCheckedIn,

    // --- User Errors ---
    #[error("The user was not found")]
    UserNotFound,

    // --- GameSystem Errors ---
    #[error("Game system name cannot be empty")]
    EmptyName,
    #[error("Game system name cannot exceed 100 characters")]
    NameTooLong,
    #[error("Game system name must be at least 2 characters long")]
    NameTooShort,

    // --- Builder Errors ---
    #[error("Missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect password")]
    IncorrectPassword,
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum InfrastructureError {
    // --- Setup Errors ---
    #[error("Failed to get environment variable: {0}")]
    FailedToGetEnvironmentVariable(String),
    #[error("Failed to bind address: {0}")]
    FailedToBindAddress(String),
    #[error("Failed to launch server: {0}")]
    FailedToLaunchServer(String),
    #[error("Failed to parse PORT to u32: {0}")]
    FailedToParsePort(String),
    #[error("Failed to establish database connection: {0}")]
    FailedToEstablishDatabaseConnection(String),
    #[error("Failed to run database migrations: {0}")]
    FailedToRunDBMigrations(String),
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheckFailed(String),
    #[error("Failed to setup server address: {0}")]
    FailedToSetupServerAddress(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Environment validation failed: {0}")]
    EnvironmentValidationFailed(String),
    #[error("Attempted to launch server without setup")]
    LaunchWithoutSetup,

    // --- Database Errors ---
    #[error("Username already taken")]
    UsernameAlreadyTaken,
    #[error("Email already taken")]
    EmailAlreadyTaken,
    #[error("Game system name already taken")]
    GameSystemNameAlreadyTaken,
    #[error("User session intent already exists")]
    UserSessionIntentAlreadyExists,
    #[error("User already member of table")]
    UserAlreadyMemberOfTable,
    #[error("User not found")]
    UserNotFound(Uuid),
    #[error("Foreign key violation in table {table} on field {field}")]
    ForeignKeyViolation { table: String, field: String },
    #[error("Game system not found")]
    GameSystemNotFound(Uuid),
    #[error("RPG table not found")]
    RpgTableNotFound(Uuid),
    #[error("Session not found")]
    SessionNotFound(Uuid),
    #[error("Session intent not found")]
    SessionIntentNotFound(Uuid),
    #[error("Unknown constraint")]
    UnknownConstraint(String),
    #[error("Not found")]
    NotFound,
    #[error("Validation error")]
    ValidationError(String),
    #[error("Invalid input")]
    InvalidInput(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Nothing to update")]
    NothingToUpdate,

    // --- Hashing Errors ---
    #[error("Failed to encode JWT: {0}")]
    FailedToEncodeJwt(String),
    #[error("Failed to decode JWT: {0}")]
    FailedToDecodeJwt(String),
    #[error("Hashing failed: {0}")]
    HashingFailed(String),
}
