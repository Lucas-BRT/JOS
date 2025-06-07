use crate::application::error::ApplicationError;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    ApplicationSetup(ApplicationSetupError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
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
        }
    }
}
