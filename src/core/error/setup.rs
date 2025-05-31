use super::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationSetupError {
    #[error("Failed to get environment variable: {0}")]
    FailedToGetEnvironmentVariable(String),
    #[error("Failed to start TCP listener: {0}")]
    FailedToStartTcpListener(String),
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

impl From<ApplicationSetupError> for AppError {
    fn from(err: ApplicationSetupError) -> Self {
        AppError::ApplicationSetup(err)
    }
}
