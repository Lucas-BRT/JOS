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
    #[error("Database error")]
    DatabaseError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Connection error")]
    ConnectionError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Migration error")]
    MigrationError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Not found: {entity}[id={id}]")]
    NotFound { entity: &'static str, id: String },
    #[error("Constraint violation: {constraint}")]
    ConstraintViolation { constraint: String },
}

impl IntoResponse for PersistenceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PersistenceError::NotFound { entity, id } => (
                StatusCode::NOT_FOUND,
                format!("Entity '{}' with id {} not found", entity, id),
            ),
            PersistenceError::ConstraintViolation { constraint } => (
                StatusCode::CONFLICT,
                format!("Conflict due to constraint: {}", constraint),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal persistence error occurred".to_string(),
            ),
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
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
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

            ApplicationError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),

            ApplicationError::InvalidInput { message } => (StatusCode::BAD_REQUEST, message),

            ApplicationError::ServiceUnavailable { service } => {
                (StatusCode::SERVICE_UNAVAILABLE, service)
            }
        };

        let body = Json(json!({

            "error": error_message,

        }));

        (status, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Business rule violation: {message}")]
    BusinessRuleViolation { message: String },
    #[error("Entity not found: {entity_type}[id={entity_id}]")]
    EntityNotFound {
        entity_type: &'static str,
        entity_id: String,
    },
    #[error("Invalid state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DomainError::BusinessRuleViolation { message } => (StatusCode::BAD_REQUEST, message),
            DomainError::EntityNotFound {
                entity_type,
                entity_id,
            } => (
                StatusCode::NOT_FOUND,
                format!("Entity '{}' with id {} not found", entity_type, entity_id),
            ),
            DomainError::InvalidStateTransition { from, to } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Invalid state transition from '{}' to '{}'", from, to),
            ),
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
