use crate::config::AppConfig;
use crate::persistence::Db;
use crate::persistence::postgres::repositories::{
    PostgresGameSystemRepository, PostgresRefreshTokenRepository, PostgresSessionRepository,
    PostgresTableMemberRepository, PostgresTableRepository, PostgresTableRequestRepository,
    PostgresUserRepository,
};
use crate::security::{BcryptPasswordProvider, JwtTokenProvider};
use application::auth_service::AuthService;
use application::game_system_service::GameSystemService;
use application::password_service::PasswordService;
use application::session_service::SessionService;
use application::table_member_service::TableMemberService;
use application::table_request_service::TableRequestService;
use application::table_service::TableService;
use application::user_service::UserService;
use axum::extract::FromRef;
use shared::Result;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub table_service: TableService,
    pub table_request_service: TableRequestService,
    pub session_service: SessionService,
    pub auth_service: AuthService,
    pub password_service: PasswordService,
    pub game_system_service: GameSystemService,
    pub table_member_service: TableMemberService,
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

impl FromRef<Arc<AppState>> for AppState {
    fn from_ref(input: &Arc<AppState>) -> Self {
        input.as_ref().clone()
    }
}

pub async fn setup_app_state(database: &Db, config: &AppConfig) -> Result<AppState> {
    let app_state = setup_services(database, config).await?;

    Ok(app_state)
}

pub async fn setup_services(database: &Db, config: &AppConfig) -> Result<AppState> {
    info!("🔧 Initializing application setup...");
    info!("📝 Logging system initialized");
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
    let table_member_repo_for_req = Arc::new(PostgresTableMemberRepository::new(database.clone()));
    let table_member_service_for_req =
        Arc::new(TableMemberService::new(table_member_repo_for_req.clone()));
    let table_request_repo = Arc::new(PostgresTableRequestRepository::new(database.clone()));
    let table_request_service = TableRequestService::new(
        table_request_repo.clone(),
        table_repo.clone(),
        table_member_repo_for_req.clone(),
        table_member_service_for_req.clone(),
    );
    info!("✅ Table request service initialized");

    // Session service
    let session_repo = Arc::new(PostgresSessionRepository::new(database.clone()));
    let session_service = SessionService::new(session_repo.clone(), table_repo.clone());
    info!("✅ Session service initialized");

    // Auth service
    let jwt_provider = Arc::new(JwtTokenProvider::new(
        config.jwt_secret.clone(),
        config.jwt_expiration_duration,
    ));
    let refresh_token_repo = Arc::new(PostgresRefreshTokenRepository::new(database.clone()));
    let auth_service = AuthService::new(
        user_repo.clone(),
        password_repo.clone(),
        jwt_provider.clone(),
        refresh_token_repo.clone(),
    );
    info!("✅ Auth service initialized");

    // Game System service
    let game_system_repo = Arc::new(PostgresGameSystemRepository::new(database.clone()));
    let game_system_service = GameSystemService::new(game_system_repo.clone());
    info!("✅ Game System service initialized");

    // Table member service
    let table_member_repo = Arc::new(PostgresTableMemberRepository::new(database.clone()));
    let table_member_service = TableMemberService::new(table_member_repo.clone());
    info!("✅ Table member service initialized");

    // Create AppState
    let app_state = AppState {
        config: config.clone(),
        user_service,
        table_service,
        table_request_service,
        session_service,
        auth_service,
        password_service,
        game_system_service,
        table_member_service,
    };

    info!("🎉 Application setup completed successfully!");

    Ok(app_state)
}
