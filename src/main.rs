#![allow(incomplete_features)]

mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod prelude;

use application::services::table_service::TableService;
use application::services::user_service::UserService;
use core::error::{AppError, ApplicationSetupError};
use core::state::AppState;
use infrastructure::config::Config;
use infrastructure::persistance::postgres::create_postgres_pool;
use infrastructure::persistance::postgres::migrations::run_postgres_migrations;
use infrastructure::persistance::postgres::repositories::PostgresTableRepository;
use infrastructure::persistance::postgres::repositories::user::PostgresUserRepository;
use interfaces::http::create_router;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let pool = create_postgres_pool(&config.database_url).await?;

    run_postgres_migrations(pool.clone()).await?;

    let user_repo = PostgresUserRepository::new(pool.clone());
    let user_service = UserService::new(Arc::new(user_repo));

    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));

    let state = AppState::new(pool, Arc::new(config), user_service, table_service);

    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| ApplicationSetupError::FailedToStartTcpListener(err.to_string()))?;

    println!(
        "server launched at: {}",
        listener.local_addr().expect("failed to get server addr")
    );

    axum::serve(listener, create_router(state))
        .await
        .map_err(|err| ApplicationSetupError::FailedToLaunchServer(err.to_string()))?;

    Ok(())
}
