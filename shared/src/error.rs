use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Persistence error: {0}")]
    Persistence(PersistenceError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Domain error: {0}")]
    Domain(DomainError),
    #[error("Setup error: {0}")]
    Setup(SetupError),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Persistence(error) => error.into_response(),
            Error::Application(error) => error.into_response(),
            Error::Domain(error) => error.into_response(),
            Error::Setup(error) => error.into_response(),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
            Error::Validation(error) => {
                (StatusCode::BAD_REQUEST, error.to_string()).into_response()
            }
        }
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

impl IntoResponse for PersistenceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PersistenceError::DatabaseError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
            PersistenceError::ConnectionError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
            PersistenceError::MigrationError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect password")]
    IncorrectPassword,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApplicationError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            ApplicationError::IncorrectPassword => {
                (StatusCode::FORBIDDEN, "Incorrect password".to_string())
            }
            ApplicationError::InvalidInput(error) => (StatusCode::BAD_REQUEST, error),
            ApplicationError::ServiceUnavailable(error) => (StatusCode::SERVICE_UNAVAILABLE, error),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
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

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DomainError::BusinessRuleViolation(error) => (StatusCode::BAD_REQUEST, error),
            DomainError::EntityNotFound(error) => (StatusCode::NOT_FOUND, error),
            DomainError::InvalidState(error) => (StatusCode::UNPROCESSABLE_ENTITY, error),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
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

impl IntoResponse for SetupError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SetupError::FailedToGetEnvironmentVariable(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            SetupError::FailedToBindAddress(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
            SetupError::FailedToLaunchServer(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
            SetupError::FailedToParsePort(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
            SetupError::FailedToEstablishDatabaseConnection(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            SetupError::FailedToRunDBMigrations(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            SetupError::DatabaseHealthCheckFailed(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            SetupError::FailedToSetupServerAddress(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            SetupError::InvalidConfiguration(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
            SetupError::EnvironmentValidationFailed(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
