use crate::application::services::table_service::TableService;
use crate::application::services::user_service::UserService;
use crate::core::error::AppError;
use crate::core::state::AppState;
use crate::infrastructure::config::Config;
use crate::infrastructure::persistance::postgres::create_postgres_pool;
use crate::infrastructure::persistance::postgres::migrations::run_postgres_migrations;
use crate::infrastructure::persistance::postgres::repositories::PostgresTableRepository;
use crate::infrastructure::persistance::postgres::repositories::user::PostgresUserRepository;
use crate::interfaces::http::create_router;
use std::sync::Arc;

use super::error::ApplicationSetupError;

pub async fn setup_services() -> Result<AppState, AppError> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let pool = create_postgres_pool(&config.database_url).await?;

    run_postgres_migrations(pool.clone()).await?;

    let user_repo = PostgresUserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repo));

    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));

    let state = AppState::new(pool, Arc::new(config), user_service, table_service);

    Ok(state)
}

pub async fn launch_server(state: AppState) -> Result<(), AppError> {
    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| {
            AppError::ApplicationSetup(ApplicationSetupError::FailedToStartTcpListener(
                err.to_string(),
            ))
        })?;

    println!(
        "server launched at: {}",
        listener.local_addr().expect("failed to get server addr")
    );

    axum::serve(listener, create_router(state))
        .await
        .map_err(|err| {
            AppError::ApplicationSetup(ApplicationSetupError::FailedToLaunchServer(err.to_string()))
        })
}
