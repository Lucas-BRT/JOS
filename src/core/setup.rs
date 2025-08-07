use super::config::Config;
use super::state::AppState;
use crate::application::services::{table_service::TableService, table_request_service::TableRequestService, user_service::UserService};
use crate::infrastructure::create_postgres_pool;
use crate::infrastructure::prelude::*;
use crate::infrastructure::repositories::prelude::TableRequestRepository;
use crate::infrastructure::run_postgres_migrations;
use crate::interfaces::http::create_router;
use crate::{Error, Result};
use std::sync::Arc;
use tracing::{Level, info};
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
}

impl From<SetupError> for Error {
    fn from(err: SetupError) -> Self {
        Error::Setup(err)
    }
}

pub async fn setup_services() -> Result<Arc<AppState>> {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(true)
        .init();

    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let pool = create_postgres_pool(&config.database_url).await?;

    run_postgres_migrations(pool.clone()).await?;

    let user_repo = UserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repo));

    let table_repo = TableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));

    let table_request_repo = TableRequestRepository::new(pool.clone());
    let table_request_service = TableRequestService::new(Arc::new(table_request_repo));

    let state = AppState::new(config, user_service, table_service, table_request_service);

    Ok(Arc::new(state))
}

pub async fn launch_server(state: Arc<AppState>) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToBindAddress(err.to_string())))?;
    info!(
        "server launched at: {}",
        listener.local_addr().expect("failed to get server addr")
    );

    axum::serve(listener, create_router(state))
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToLaunchServer(err.to_string())))
}
