use crate::Error;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SetupError {
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
    #[error("Database health check failed: {0}")]
    DatabaseHealthCheckFailed(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Environment validation failed: {0}")]
    EnvironmentValidationFailed(String),
}

impl SetupError {
    pub fn user_friendly_message(&self) -> String {
        match self {
            SetupError::FailedToGetEnvironmentVariable(var) => {
                format!(
                    "❌ Missing environment variable: {var}\n\n\
                    💡 Solution:\n\
                    • Check if your .env file exists in the project root\n\
                    • Verify that {var} is defined in your .env file\n\
                    • Example: {var}=your_value\n\
                    • Run './scripts/setup.sh' to create a .env template"
                )
            }
            SetupError::FailedToEstablishDatabaseConnection(err) => {
                let mut message = format!("❌ Database connection failed: {err}\n\n");

                if err.contains("password authentication failed") {
                    message.push_str(
                        "🔐 Authentication Error:\n\
                        • Check your DATABASE_URL in .env file\n\
                        • Verify username and password are correct\n\
                        • Example: postgres://username:password@localhost:5432/db_name\n\
                        • Make sure PostgreSQL is running\n\
                        • Try: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine\n\n"
                    );
                } else if err.contains("connection refused") {
                    message.push_str(
                        "🔌 Connection Error:\n\
                        • PostgreSQL is not running\n\
                        • Check if PostgreSQL is started\n\
                        • Verify the port in DATABASE_URL (default: 5432)\n\
                        • Try: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine\n\n"
                    );
                } else if err.contains("database") && err.contains("does not exist") {
                    message.push_str(
                        "🗄️ Database Error:\n\
                        • Database does not exist\n\
                        • Create the database: CREATE DATABASE jos_db;\n\
                        • Or use Docker: docker run --name jos-postgres -e POSTGRES_DB=jos_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:16-alpine\n\n"
                    );
                }

                message.push_str(
                    "💡 Troubleshooting:\n\
                    • Run 'cargo run -p jos-cli setup' to check your environment\n\
                    • Verify DATABASE_URL format: postgres://user:pass@host:port/db\n\
                    • Check PostgreSQL logs for more details\n\
                    • Ensure firewall allows connections to PostgreSQL port",
                );

                message
            }
            SetupError::FailedToBindAddress(err) => {
                format!(
                    "❌ Failed to bind server to address: {}\n\n\
                    💡 Solution:\n\
                    • Port might be in use by another application\n\
                    • Check if port {} is available\n\
                    • Try a different port in your .env file\n\
                    • Example: PORT=3001\n\
                    • Use 'lsof -i :PORT' to see what's using the port",
                    err,
                    std::env::var("PORT").unwrap_or_else(|_| "3000".to_string())
                )
            }
            SetupError::FailedToParsePort(err) => {
                format!(
                    "❌ Invalid PORT value: {err}\n\n\
                    💡 Solution:\n\
                    • PORT must be a number between 1024 and 65535\n\
                    • Check your .env file\n\
                    • Example: PORT=3000\n\
                    • Common ports: 3000, 8080, 5000"
                )
            }
            SetupError::FailedToRunDBMigrations(err) => {
                format!(
                    "❌ Database migration failed: {err}\n\n\
                    💡 Solution:\n\
                    • Check if database exists and is accessible\n\
                    • Verify user has permissions to create tables\n\
                    • Run 'sqlx migrate run' manually to see detailed errors\n\
                    • Check PostgreSQL logs\n\
                    • Ensure DATABASE_URL is correct in .env file"
                )
            }
            SetupError::DatabaseHealthCheckFailed(err) => {
                format!(
                    "❌ Database health check failed: {err}\n\n\
                    💡 Solution:\n\
                    • Database connection is not working properly\n\
                    • Check if PostgreSQL is running\n\
                    • Verify DATABASE_URL in .env file\n\
                    • Try connecting manually: psql DATABASE_URL\n\
                    • Check PostgreSQL logs for errors"
                )
            }
            SetupError::InvalidConfiguration(err) => {
                format!(
                    "❌ Invalid configuration: {err}\n\n\
                    💡 Solution:\n\
                    • Check your .env file for correct values\n\
                    • Verify all required variables are set\n\
                    • Run 'cargo run -p jos-cli setup' to validate your setup\n\
                    • See docs/SETUP.md for configuration examples"
                )
            }
            SetupError::EnvironmentValidationFailed(err) => {
                format!(
                    "❌ Environment validation failed: {err}\n\n\
                    💡 Solution:\n\
                    • Check if .env file exists in project root\n\
                    • Verify all required variables are set\n\
                    • Run './scripts/setup.sh' to create .env template\n\
                    • Required variables: DATABASE_URL, PORT, JWT_SECRET\n\
                    • See docs/SETUP.md for configuration guide"
                )
            }
            SetupError::FailedToSetupServerAddress(err) => {
                format!(
                    "❌ Failed to setup server address: {err}\n\n\
                    💡 Solution:\n\
                    • Check PORT value in .env file\n\
                    • PORT must be a valid number\n\
                    • Try: PORT=3000\n\
                    • Ensure port is between 1024 and 65535"
                )
            }
            SetupError::FailedToLaunchServer(err) => {
                format!(
                    "❌ Failed to launch server: {err}\n\n\
                    💡 Solution:\n\
                    • Check if port is available\n\
                    • Verify server configuration\n\
                    • Check system resources\n\
                    • Try restarting the application"
                )
            }
        }
    }
}

impl From<SetupError> for Error {
    fn from(err: SetupError) -> Self {
        Error::Setup(err)
    }
}
