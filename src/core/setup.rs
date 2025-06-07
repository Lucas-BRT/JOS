use super::config::Config;
use super::error::ApplicationSetupError;
use super::state::AppState;
use crate::application::services::table_service::TableService;
use crate::application::services::user_service::UserService;
use crate::infrastructure::persistance::postgres::create_postgres_pool;
use crate::infrastructure::persistance::postgres::migrations::run_postgres_migrations;
use crate::infrastructure::persistance::postgres::repositories::PostgresTableRepository;
use crate::infrastructure::persistance::postgres::repositories::user::PostgresUserRepository;
use crate::interfaces::http::create_router;
use crate::{Error, Result};
use std::sync::Arc;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::format::FmtSpan;

pub async fn setup_services() -> Result<Arc<AppState>> {
    let _ = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(true)
        .init();

    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let pool = create_postgres_pool(&config.database_url).await?;

    run_postgres_migrations(pool.clone()).await?;

    let user_repo = PostgresUserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repo));

    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));

    let state = AppState::new(config, user_service, table_service);

    Ok(Arc::new(state))
}

pub async fn launch_server(state: Arc<AppState>) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| {
            Error::ApplicationSetup(ApplicationSetupError::FailedToBindAddress(err.to_string()))
        })?;
    info!(
        "server launched at: {}",
        listener.local_addr().expect("failed to get server addr")
    );

    axum::serve(listener, create_router(state))
        .await
        .map_err(|err| {
            Error::ApplicationSetup(ApplicationSetupError::FailedToLaunchServer(err.to_string()))
        })
}
