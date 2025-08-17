mod database;
mod env;
mod errors;
mod logging;
mod server;

use crate::Result;
use crate::application::*;
use crate::core::config::Config;
use crate::core::state::AppState;
use crate::infrastructure::prelude::*;
use crate::infrastructure::{
    create_postgres_pool,
    repositories::{jwt::JwtTokenProvider, table_request::PostgresTableRequestRepository, user::PostgresUserRepository},
    run_postgres_migrations,
};
pub use errors::SetupError;
pub use server::launch_server;
use std::sync::Arc;
use tracing::{info, warn};

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

    // User service
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let user_service = UserService::new(user_repo.clone());
    info!("✅ User service initialized");

    // Password service
    let password_repo = Arc::new(Argon2PasswordProvider::default());
    let password_service = PasswordService::new(password_repo.clone());
    info!("✅ Password service initialized");

    // Table service
    let table_repo = Arc::new(TableRepository::new(pool.clone()));
    let table_service = TableService::new(table_repo.clone());
    info!("✅ Table service initialized");

    // Table request service
    let table_request_repo = Arc::new(PostgresTableRequestRepository::new(pool.clone()));
    let table_request_service = TableRequestService::new(table_request_repo.clone(), table_repo.clone());
    info!("✅ Table request service initialized");

    // Auth service
    let jwt_provider = Arc::new(JwtTokenProvider::new(
        config.jwt_secret.clone(),
        config.jwt_expiration_duration,
    ));
    let auth_service = AuthService::new(
        user_repo.clone(),
        password_repo.clone(),
        jwt_provider.clone(),
    );
    info!("✅ Auth service initialized");

    let state = AppState::new(
        config,
        user_service,
        table_service,
        table_request_service,
        auth_service,
        password_service,
    );
    info!("✅ Application state initialized");

    info!("🎉 Application setup completed successfully!");
    Ok(Arc::new(state))
}
