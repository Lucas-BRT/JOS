use axum::extract::FromRef;

use crate::adapters::outbound::postgres::repositories::{
    PostgresSessionRepository, PostgresTableRepository, PostgresTableRequestRepository,
    PostgresUserRepository,
};
use crate::adapters::outbound::{BcryptPasswordProvider, JwtTokenProvider};
use crate::application::auth_service::AuthService;
use crate::application::password_service::PasswordService;
use crate::application::search_service::SearchService;
use crate::application::session_service::SessionService;
use crate::application::table_request_service::TableRequestService;
use crate::application::{table_service::TableService, user_service::UserService};
use crate::infrastructure::config::AppConfig;
use crate::{Db, Result};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
    pub session_service: SessionService,
    pub search_service: SearchService,
    pub auth_service: AuthService,
    pub password_service: PasswordService,
}

impl FromRef<AppState> for AppConfig {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for UserService {
    fn from_ref(input: &AppState) -> Self {
        input.user_service.clone()
    }
}

impl FromRef<AppState> for TableService {
    fn from_ref(input: &AppState) -> Self {
        input.table_service.clone()
    }
}

impl FromRef<AppState> for TableRequestService {
    fn from_ref(input: &AppState) -> Self {
        input.table_request_service.clone()
    }
}

impl FromRef<AppState> for SessionService {
    fn from_ref(input: &AppState) -> Self {
        input.session_service.clone()
    }
}

impl FromRef<AppState> for SearchService {
    fn from_ref(input: &AppState) -> Self {
        input.search_service.clone()
    }
}

impl FromRef<AppState> for AuthService {
    fn from_ref(input: &AppState) -> Self {
        input.auth_service.clone()
    }
}

impl FromRef<AppState> for PasswordService {
    fn from_ref(input: &AppState) -> Self {
        input.password_service.clone()
    }
}

pub async fn setup_app_state(database: &Db) -> Result<AppState> {
    let config = AppConfig::from_env()?;
    config.validate_config()?;
    config.display_startup_info();

    let app_state = setup_services(database).await?;

    Ok(app_state)
}

pub async fn setup_services(database: &Db) -> Result<AppState> {
    info!("🔧 Initializing application setup...");
    info!("📝 Logging system initialized");

    let config = AppConfig::from_env()?;
    config.validate_config()?;
    config.display_startup_info();

    info!("🏗️  Initializing services...");

    // User service
    let user_repo = Arc::new(PostgresUserRepository::new(database.clone()));
    let user_service = UserService::new(user_repo.clone());
    info!("✅ User service initialized");

    // Password service
    let password_repo = Arc::new(BcryptPasswordProvider);
    let password_service = PasswordService::new(password_repo.clone());
    info!("✅ Password service initialized");

    // Table service
    let table_repo = Arc::new(PostgresTableRepository::new(database.clone()));
    let table_service = TableService::new(table_repo.clone());
    info!("✅ Table service initialized");

    // Table request service
    let table_request_repo = Arc::new(PostgresTableRequestRepository::new(database.clone()));
    let table_request_service = TableRequestService::new(table_request_repo.clone());
    info!("✅ Table request service initialized");

    // Session service
    let session_repo = Arc::new(PostgresSessionRepository::new(database.clone()));
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

    info!("🎉 Application setup completed successfully!");

    Ok(app_state)
}
