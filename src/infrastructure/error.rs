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
