use crate::{
    application::error::ApplicationError,
    infrastructure::persistance::postgres::repositories::error::RepositoryError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    ApplicationSetup(ApplicationSetupError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
    #[error("Repository error: {0}")]
    Repository(RepositoryError),
}

impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        match self {
            RepositoryError::NotFound(message) => {
                tracing::error!("Not found error: {}", message);
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response();
            }
            RepositoryError::UniqueViolation(message) => {
                tracing::error!("Unique violation error: {}", message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response();
            }
            RepositoryError::ForeignKeyViolation(message) => {
                tracing::error!("Foreign key violation error: {}", message);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": message,

                    })),
                )
                    .into_response();
            }
            RepositoryError::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Internal Server Error"
                    })),
                )
                    .into_response();
            }
            RepositoryError::ConnectionError(message) => {
                tracing::error!("Connection error: {}", message);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Internal Server Error"
                    })),
                )
                    .into_response();
            }
            RepositoryError::Unexpected(message) => {
                tracing::error!("Unexpected error: {}", message);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Internal Server Error"
                    })),
                )
                    .into_response();
            }
            RepositoryError::UsernameAlreadyTaken(username) => {
                tracing::error!("Username already taken: {}", username);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Username already taken",
                        "value": username
                    })),
                )
                    .into_response();
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationErrors {
    #[error("password and password_confirmation mismatch")]
    PasswordConfirmationMismatch,

    #[error("{0}")]
    Other(#[from] validator::ValidationErrors),
}

impl IntoResponse for ValidationErrors {
    fn into_response(self) -> Response {
        match self {
            ValidationErrors::PasswordConfirmationMismatch => {
                tracing::error!("Password confirmation mismatch");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "password_confirmation": ["Passwords do not match"]
                    })),
                )
                    .into_response();
            }
            ValidationErrors::Other(errors) => {
                let errors = errors
                    .errors()
                    .clone()
                    .into_keys()
                    .map(|key| key.to_string())
                    .collect::<Vec<String>>();

                tracing::error!("Validation error: {:?}", errors);

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "validation": errors
                    })),
                )
                    .into_response()
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationSetupError {
    #[error("Failed to get environment variable: {0}")]
    FailedToGetEnvironmentVariable(String),
    #[error("Failed to bind address: {0}")]
    FailedToBindAddress(String),
    #[error("Failed to launch server: {0}")]
    FailedToLaunchServer(String),
    #[error("Failed to parse PORT to u32: {0}")]
    FailedToParsePort(String),
    #[error("Failed to setup server address: {0}")]
    FailedToSetupServerAddress(String),
    #[error("Failed to establish database connection: {0}")]
    FailedToEstablishDatabaseConnection(String),
    #[error("Failed to run database migrations: {0}")]
    FailedToRunDBMigrations(String),
}

impl From<ApplicationSetupError> for Error {
    fn from(err: ApplicationSetupError) -> Self {
        Error::ApplicationSetup(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::ApplicationSetup(err) => {
                tracing::error!("Application setup error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Application(err) => {
                tracing::error!("Application error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Repository(err) => err.into_response(),
            Error::Validation(err) => err.into_response(),
        }
    }
}
