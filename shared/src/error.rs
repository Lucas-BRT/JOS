use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use log::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Persistence error: {0}")]
    Persistence(#[from] PersistenceError),
    #[error("Application error: {0}")]
    Application(#[from] ApplicationError),
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Setup error: {0}")]
    Setup(#[from] SetupError),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Persistence(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::Application(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::Domain(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::Setup(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            Error::Validation(error) => (StatusCode::BAD_REQUEST, error.to_string() ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Migration error: {0}")]
    MigrationError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
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
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Required field missing: {0}")]
    RequiredFieldMissing(String),
}
