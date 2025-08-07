use super::config::Config;
use super::state::AppState;
use crate::application::services::{table_service::TableService, table_request_service::TableRequestService, user_service::UserService};
use crate::infrastructure::create_postgres_pool;
use crate::infrastructure::prelude::*;
use crate::infrastructure::repositories::prelude::TableRequestRepository;
use crate::infrastructure::run_postgres_migrations;
use sqlx::PgPool;
use crate::interfaces::http::create_router;
use crate::{Error, Result};
use std::sync::Arc;

use tracing::{Level, info, warn, error};
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::format::FmtSpan;

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
    /// Returns a user-friendly error message with troubleshooting tips
    pub fn user_friendly_message(&self) -> String {
        match self {
            SetupError::FailedToGetEnvironmentVariable(var) => {
                format!(
                    "❌ Missing environment variable: {}\n\n\
                    💡 Solution:\n\
                    • Check if your .env file exists in the project root\n\
                    • Verify that {} is defined in your .env file\n\
                    • Example: {}=your_value\n\
                    • Run './scripts/setup.sh' to create a .env template",
                    var, var, var
                )
            }
            SetupError::FailedToEstablishDatabaseConnection(err) => {
                let mut message = format!("❌ Database connection failed: {}\n\n", err);
                
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
                    • Ensure firewall allows connections to PostgreSQL port"
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
                    "❌ Invalid PORT value: {}\n\n\
                    💡 Solution:\n\
                    • PORT must be a number between 1024 and 65535\n\
                    • Check your .env file\n\
                    • Example: PORT=3000\n\
                    • Common ports: 3000, 8080, 5000",
                    err
                )
            }
            SetupError::FailedToRunDBMigrations(err) => {
                format!(
                    "❌ Database migration failed: {}\n\n\
                    💡 Solution:\n\
                    • Check if database exists and is accessible\n\
                    • Verify user has permissions to create tables\n\
                    • Run 'sqlx migrate run' manually to see detailed errors\n\
                    • Check PostgreSQL logs\n\
                    • Ensure DATABASE_URL is correct in .env file",
                    err
                )
            }
            SetupError::DatabaseHealthCheckFailed(err) => {
                format!(
                    "❌ Database health check failed: {}\n\n\
                    💡 Solution:\n\
                    • Database connection is not working properly\n\
                    • Check if PostgreSQL is running\n\
                    • Verify DATABASE_URL in .env file\n\
                    • Try connecting manually: psql DATABASE_URL\n\
                    • Check PostgreSQL logs for errors",
                    err
                )
            }
            SetupError::InvalidConfiguration(err) => {
                format!(
                    "❌ Invalid configuration: {}\n\n\
                    💡 Solution:\n\
                    • Check your .env file for correct values\n\
                    • Verify all required variables are set\n\
                    • Run 'cargo run -p jos-cli setup' to validate your setup\n\
                    • See docs/SETUP.md for configuration examples",
                    err
                )
            }
            SetupError::EnvironmentValidationFailed(err) => {
                format!(
                    "❌ Environment validation failed: {}\n\n\
                    💡 Solution:\n\
                    • Check if .env file exists in project root\n\
                    • Verify all required variables are set\n\
                    • Run './scripts/setup.sh' to create .env template\n\
                    • Required variables: DATABASE_URL, PORT, JWT_SECRET\n\
                    • See docs/SETUP.md for configuration guide",
                    err
                )
            }
            SetupError::FailedToSetupServerAddress(err) => {
                format!(
                    "❌ Failed to setup server address: {}\n\n\
                    💡 Solution:\n\
                    • Check PORT value in .env file\n\
                    • PORT must be a valid number\n\
                    • Try: PORT=3000\n\
                    • Ensure port is between 1024 and 65535",
                    err
                )
            }
            SetupError::FailedToLaunchServer(err) => {
                format!(
                    "❌ Failed to launch server: {}\n\n\
                    💡 Solution:\n\
                    • Check if port is available\n\
                    • Verify server configuration\n\
                    • Check system resources\n\
                    • Try restarting the application",
                    err
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

/// Validates that all required environment variables are present
fn validate_environment() -> Result<()> {
    let required_vars = vec![
        "DATABASE_URL",
        "PORT", 
        "JWT_SECRET"
    ];

    let mut missing_vars = Vec::new();
    
    for var in required_vars {
        if std::env::var(var).is_err() {
            missing_vars.push(var);
        }
    }

    if !missing_vars.is_empty() {
        return Err(Error::Setup(SetupError::EnvironmentValidationFailed(
            format!("Missing required environment variables: {}", missing_vars.join(", "))
        )));
    }

    Ok(())
}

/// Performs a health check on the database connection
async fn health_check_database(pool: &Arc<PgPool>) -> Result<()> {
    // Test basic connectivity
    let result = sqlx::query("SELECT 1")
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => {
            info!("✅ Database health check passed");
            Ok(())
        }
        Err(e) => {
            error!("❌ Database health check failed: {}", e);
            Err(Error::Setup(SetupError::DatabaseHealthCheckFailed(e.to_string())))
        }
    }
}

/// Validates the configuration and provides helpful feedback
fn validate_config(config: &Config) -> Result<()> {
    // Validate database URL format
    if !config.database_url.starts_with("postgres://") && !config.database_url.starts_with("postgresql://") {
        return Err(Error::Setup(SetupError::InvalidConfiguration(
            "DATABASE_URL must start with 'postgres://' or 'postgresql://'".to_string()
        )));
    }

    // Validate port range (port is already validated in Config::from_env)
    // This is just for additional safety

    // Validate JWT secret length
    if config.jwt_secret.len() < 32 {
        warn!("⚠️  JWT_SECRET is shorter than recommended (32+ characters)");
    }

    info!("✅ Configuration validation passed");
    Ok(())
}

/// Displays startup information
fn display_startup_info(config: &Config) {
    info!("🚀 Starting JOS (Join Our Session) API");
    info!("📊 Environment: {}", if cfg!(debug_assertions) { "Development" } else { "Production" });
    info!("🌐 Server will bind to: {}", config.addr);
    info!("🗄️  Database: {}", config.database_url.split('@').last().unwrap_or("unknown"));
    info!("🔐 JWT expiration: {} days", config.jwt_expiration_duration.num_days());
}

pub async fn setup_services() -> Result<Arc<AppState>> {
    info!("🔧 Initializing application setup...");

    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(true)
        .init();

    info!("📝 Logging system initialized");

    // Load environment variables
    match dotenvy::dotenv() {
        Ok(_) => info!("✅ Environment variables loaded from .env file"),
        Err(_) => warn!("⚠️  No .env file found, using system environment variables"),
    }

    // Validate environment
    validate_environment()?;
    info!("✅ Environment validation passed");

    // Load and validate configuration
    let config = Config::from_env()?;
    validate_config(&config)?;
    display_startup_info(&config);

    // Initialize database connection
    info!("🔌 Establishing database connection...");
    let pool = create_postgres_pool(&config.database_url).await?;
    info!("✅ Database connection established");

    // Health check database
    health_check_database(&pool).await?;

    // Run migrations
    info!("🔄 Running database migrations...");
    run_postgres_migrations(pool.clone()).await?;
    info!("✅ Database migrations completed");

    // Initialize repositories and services
    info!("🏗️  Initializing services...");
    
    let user_repo = UserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repo));
    info!("✅ User service initialized");

    let table_repo = TableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));
    info!("✅ Table service initialized");

    let table_request_repo = TableRequestRepository::new(pool.clone());
    let table_request_service = TableRequestService::new(Arc::new(table_request_repo));
    info!("✅ Table request service initialized");

    let state = AppState::new(config, user_service, table_service, table_request_service);
    info!("✅ Application state initialized");

    info!("🎉 Application setup completed successfully!");
    Ok(Arc::new(state))
}

pub async fn launch_server(state: Arc<AppState>) -> Result<()> {
    info!("🚀 Launching HTTP server...");
    
    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToBindAddress(err.to_string())))?;
    
    let local_addr = listener.local_addr()
        .expect("failed to get server addr");
    
    info!("✅ Server bound to: {}", local_addr);
    info!("🌐 API documentation available at: http://{}/docs", local_addr);
    info!("🔍 Health check available at: http://{}/health", local_addr);

    // Graceful shutdown handling
    let router = create_router(state);
    
    axum::serve(listener, router)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToLaunchServer(err.to_string())))
}
