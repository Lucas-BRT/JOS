mod errors;
mod env;
mod database;
mod logging;
mod server;

use std::sync::Arc;
use tracing::{info, warn};
use crate::application::services::{
    jwt_service::JwtService,
    table_request_service::TableRequestService,
    table_service::TableService,
    user_service::UserService,
};
use crate::infrastructure::{create_postgres_pool, run_postgres_migrations};
use crate::infrastructure::prelude::*;
use crate::infrastructure::repositories::prelude::TableRequestRepository;
use crate::Result;
use crate::core::config::Config;
use crate::core::state::AppState;
pub use errors::SetupError;
pub use server::launch_server;


pub async fn setup_services() -> Result<Arc<AppState>> {
    logging::init_logging();

    info!("🔧 Initializing application setup...");
    info!("📝 Logging system initialized");

    match dotenvy::dotenv() {
        Ok(_) => info!("✅ Environment variables loaded from .env file"),
        Err(_) => warn!("⚠️  No .env file found, using system environment variables"),
    }

    env::validate_environment()?;
    info!("✅ Environment validation passed");

    let config = Config::from_env()?;
    config.validate_config()?;
    config.display_startup_info();

    info!("🔌 Establishing database connection...");
    let pool = create_postgres_pool(&config.database_url).await?;
    info!("✅ Database connection established");

    database::health_check_database(&pool).await?;

    info!("🔄 Running database migrations...");
    run_postgres_migrations(pool.clone()).await?;
    info!("✅ Database migrations completed");

    info!("🏗️  Initializing services...");

    let user_repo = UserRepository::new(pool.clone());
    let jwt_service = JwtService::new(
        config.jwt_secret.clone(),
        config.jwt_expiration_duration,
    );
    let user_service = UserService::new(Arc::new(user_repo), jwt_service.clone());
    info!("✅ User service initialized");

    let table_repo = TableRepository::new(pool.clone());
    let table_service = TableService::new(Arc::new(table_repo));
    info!("✅ Table service initialized");

    let table_request_repo = TableRequestRepository::new(pool.clone());
    let table_request_service = TableRequestService::new(Arc::new(table_request_repo));
    info!("✅ Table request service initialized");

    info!("✅ JWT service initialized");

    let state = AppState::new(
        config,
        user_service,
        table_service,
        table_request_service,
        jwt_service,
    );
    info!("✅ Application state initialized");

    info!("🎉 Application setup completed successfully!");
    Ok(Arc::new(state))
}


