#![allow(incomplete_features)]

use core::{
    config::Config,
    error::{AppError, ApplicationSetupError},
};

use infra::{
    db::postgres::{create_postgres_pool, migrations::run_postgres_migrations},
    web::create_router,
};

mod application;
mod core;
mod domain;
mod infra;
mod prelude;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    let pool = create_postgres_pool(config.database_url).await?;

    run_postgres_migrations(&pool).await?;

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .map_err(|err| ApplicationSetupError::FailedToStartTcpListener(err.to_string()))?;

    println!(
        "server launched at: {}",
        listener.local_addr().expect("failed to get server addr")
    );

    axum::serve(listener, create_router(pool))
        .await
        .map_err(|err| ApplicationSetupError::FailedToLaunchServer(err.to_string()))?;

    Ok(())
}
