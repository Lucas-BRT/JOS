use shared::error::{Error, SetupError as SharedSetupError};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
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

impl From<SetupError> for Error {
    fn from(err: SetupError) -> Self {
        match err {
            SetupError::FailedToGetEnvironmentVariable(msg) => {
                Error::Setup(SharedSetupError::FailedToGetEnvironmentVariable(msg))
            }
            SetupError::FailedToBindAddress(msg) => {
                Error::Setup(SharedSetupError::FailedToBindAddress(msg))
            }
            SetupError::FailedToLaunchServer(msg) => {
                Error::Setup(SharedSetupError::FailedToLaunchServer(msg))
            }
            SetupError::FailedToParsePort(msg) => {
                Error::Setup(SharedSetupError::FailedToParsePort(msg))
            }
            SetupError::FailedToEstablishDatabaseConnection(msg) => {
                Error::Setup(SharedSetupError::FailedToEstablishDatabaseConnection(msg))
            }
            SetupError::FailedToRunDBMigrations(msg) => {
                Error::Setup(SharedSetupError::FailedToRunDBMigrations(msg))
            }
            SetupError::DatabaseHealthCheckFailed(msg) => {
                Error::Setup(SharedSetupError::DatabaseHealthCheckFailed(msg))
            }
            SetupError::FailedToSetupServerAddress(msg) => {
                Error::Setup(SharedSetupError::FailedToSetupServerAddress(msg))
            }
            SetupError::InvalidConfiguration(msg) => {
                Error::Setup(SharedSetupError::InvalidConfiguration(msg))
            }
            SetupError::EnvironmentValidationFailed(msg) => {
                Error::Setup(SharedSetupError::EnvironmentValidationFailed(msg))
            }
        }
    }
}
