use crate::config::AppConfig;
use crate::persistence::Db;
use crate::persistence::postgres::repositories::*;
use crate::security::*;
use crate::state::AppState;
use application::*;
use axum::Router;
use shared::Error;
use shared::error::InfrastructureError;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct Server {
    router: Router,
    state: Option<AppState>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            state: None,
        }
    }

    pub async fn setup_services(
        mut self,
        database: &Db,
        config: &AppConfig,
    ) -> Result<Self, Error> {
        info!("🔧 Initializing application setup...");
        info!("📝 Logging system initialized");
        info!("🏗️  Initializing services...");

        // User service
        let user_repo = PostgresUserRepository::new(database.clone());
        let user_service = UserService::new(user_repo.clone());
        info!("✅ User service initialized");

        // Password service
        let password_repo = BcryptPasswordProvider;
        let password_service = PasswordService::new(password_repo.clone());
        info!("✅ Password service initialized");

        // Table service
        let table_repo = PostgresTableRepository::new(database.clone());
        let table_service = TableService::new(table_repo.clone());
        info!("✅ Table service initialized");

        // Table request service
        let table_request_repo = PostgresTableRequestRepository::new(database.clone());
        let table_request_service =
            TableRequestService::new(table_request_repo.clone(), table_repo.clone());
        info!("✅ Table request service initialized");

        // Session service
        let session_repo = PostgresSessionRepository::new(database.clone());
        let session_service = SessionService::new(session_repo.clone(), table_repo.clone());
        info!("✅ Session service initialized");

        // Auth service
        let jwt_provider =
            JwtTokenProvider::new(config.jwt_secret.clone(), config.jwt_expiration_duration);
        let refresh_token_repo = PostgresRefreshTokenRepository::new(database.clone());
        let auth_service = AuthService::new(
            user_repo.clone(),
            password_repo.clone(),
            jwt_provider.clone(),
            refresh_token_repo.clone(),
        );
        info!("✅ Auth service initialized");

        // Game System service
        let game_system_repo = PostgresGameSystemRepository::new(database.clone());
        let game_system_service = GameSystemService::new(game_system_repo.clone());
        info!("✅ Game System service initialized");

        // Table member service
        let table_member_repo = PostgresTableMemberRepository::new(database.clone());
        let table_member_service = TableMemberService::new(table_member_repo.clone());
        info!("✅ Table member service initialized");

        let state = AppState {
            config: Arc::new(config.clone()),
            user_service: Arc::new(user_service.clone()),
            table_service: Arc::new(table_service.clone()),
            table_request_service: Arc::new(table_request_service.clone()),
            session_service: Arc::new(session_service.clone()),
            auth_service: Arc::new(auth_service.clone()),
            password_service: Arc::new(password_service.clone()),
            game_system_service: Arc::new(game_system_service.clone()),
            table_member_service: Arc::new(table_member_service.clone()),
        };

        self.state = Some(state);

        info!("🎉 setup completed successfully!");

        Ok(self)
    }

    pub async fn launch(self) -> Result<(), Error> {
        let state = match self.state {
            Some(state) => state,
            None => {
                return Err(Error::Infrastructure(
                    InfrastructureError::LaunchWithoutSetup,
                ));
            }
        };

        info!("🚀 Launching HTTP server...");

        let listener = TcpListener::bind(&state.config.addr).await.map_err(|err| {
            Error::Infrastructure(InfrastructureError::FailedToBindAddress(err.to_string()))
        })?;

        let local_addr = listener.local_addr().expect("failed to get server addr");

        info!("✅ Server bound to: {}", local_addr);
        info!(
            "🌐 API documentation available at: http://{}/docs",
            local_addr
        );
        info!("🔍 Health check available at: http://{}/health", local_addr);

        axum::serve(listener, self.router).await.map_err(|err| {
            Error::Infrastructure(InfrastructureError::FailedToLaunchServer(err.to_string()))
        })
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
