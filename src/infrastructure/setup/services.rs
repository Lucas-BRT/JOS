use crate::{
    Result,
    adapters::outbound::{
        BcryptPasswordProvider, JwtTokenProvider,
        postgres::{
            create_postgres_pool,
            repositories::{
                PostgresSessionRepository, PostgresTableRepository, PostgresTableRequestRepository,
                PostgresUserRepository,
            },
            run_postgres_migrations,
        },
    },
    application::{
        AuthService, PasswordService, SearchService, SessionService, TableRequestService,
        TableService, UserService,
    },
    infrastructure::{AppState, setup::config::Config},
};
use axum::Router;
use std::sync::Arc;
use tracing::{info, warn};

pub async fn setup_services() -> Result<(Router, AppState)> {
    info!("🔧 Initializing application setup...");
    info!("📝 Logging system initialized");

    match dotenvy::dotenv() {
        Ok(_) => info!("✅ Environment variables loaded from .env file"),
        Err(_) => warn!("⚠️  No .env file found, using system environment variables"),
    }

    let config = Config::from_env()?;
    config.validate_config()?;
    config.display_startup_info();

    info!("🔌 Establishing database connection...");
    let pool = create_postgres_pool(&config.database_url).await?;
    info!("✅ Database connection established");

    info!("🔄 Running database migrations...");
    run_postgres_migrations(pool.clone()).await?;
    info!("✅ Database migrations completed");

    info!("🏗️  Initializing services...");

    // User service
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let user_service = UserService::new(user_repo.clone());
    info!("✅ User service initialized");

    // Password service
    let password_repo = Arc::new(BcryptPasswordProvider);
    let password_service = PasswordService::new(password_repo.clone());
    info!("✅ Password service initialized");

    // Table service
    let table_repo = Arc::new(PostgresTableRepository::new(pool.clone()));
    let table_service = TableService::new(table_repo.clone());
    info!("✅ Table service initialized");

    // Table request service
    let table_request_repo = Arc::new(PostgresTableRequestRepository::new(pool.clone()));
    let table_request_service = TableRequestService::new(table_request_repo.clone());
    info!("✅ Table request service initialized");

    // Session service
    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let session_service = SessionService::new(session_repo.clone());
    info!("✅ Session service initialized");

    // Search service
    let search_service = SearchService::new(user_repo.clone(), table_repo.clone());
    info!("✅ Search service initialized");

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

    // Create AppState
    let app_state = AppState {
        config: config.clone(),
        user_service,
        table_service,
        table_request_service,
        session_service,
        search_service,
        auth_service,
        password_service,
    };

    // Create router with AppState
    let router = Router::new().with_state(app_state.clone());

    info!("🎉 Application setup completed successfully!");

    Ok((router, app_state))
}
