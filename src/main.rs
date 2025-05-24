#![allow(incomplete_features)]

use error::AppError;
use infra::{
    db::postgres::{create_postgres_pool, migrations::run_postgres_migrations},
    web::create_router,
};

mod application;
mod config;
mod domain;
mod error;
mod infra;
mod prelude;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;

    let pool = create_postgres_pool(config.database_url)
        .await
        .expect("Failed to create PostgreSQL pool");

    run_postgres_migrations(&pool)
        .await
        .expect("failed to run Postgres migrations");

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("msg");

    println!("server launched at: {}", listener.local_addr().unwrap());

    axum::serve(listener, create_router(pool))
        .await
        .expect("failed to launch server");

    Ok(())
}
