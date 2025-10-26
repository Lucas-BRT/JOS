use api::http::dtos::LoginResponse;
use axum_test::TestServer;
use chrono::Duration;
use dotenvy::dotenv;
use jos::{
    application::*,
    domain::{auth::Authenticator, entities::*},
    infrastructure::{
        config::AppConfig, persistence::postgres::repositories::*, security::*,
        setup::environment::Environment, state::AppState,
    },
};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use wiremock::MockServer;

pub const DEFAULT_USER_PASSWORD: &str = "Password123!";

fn config_for_test(_pool: &PgPool) -> AppConfig {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

    AppConfig {
        database_url,
        addr: std::net::SocketAddr::from_str("127.0.0.1:8080").unwrap(),
        jwt_secret: "test-secret".to_string(),
        jwt_expiration_duration: Duration::days(1),
        environment: Environment::Development,
    }
}

#[derive(Default)]
pub struct SeededData {
    pub users: HashMap<String, User>,
    pub tables: HashMap<String, Table>,
    pub game_systems: HashMap<String, GameSystem>,
    pub sessions: HashMap<String, Session>,
}

pub struct TestEnvironment {
    pub server: TestServer,
    pub state: Arc<AppState>,
    pub seeded: SeededData,
    pub mock_server: MockServer,
}

struct UserSeedOptions {
    identifier: String,
    username: String,
    email: String,
}

struct TableSeedOptions {
    identifier: String,
    title: String,
    owner_identifier: String,
}

struct SessionSeedOptions {
    identifier: String,
    name: String,
    table_identifier: String,
}

struct GameSystemSeedOptions {
    identifier: String,
    name: String,
}

pub struct TestEnvironmentBuilder {
    pool: PgPool,
    users_to_seed: Vec<UserSeedOptions>,
    tables_to_seed: Vec<TableSeedOptions>,
    sessions_to_seed: Vec<SessionSeedOptions>,
    game_systems_to_seed: Vec<GameSystemSeedOptions>,
}

impl TestEnvironmentBuilder {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            users_to_seed: Vec::new(),
            tables_to_seed: Vec::new(),
            sessions_to_seed: Vec::new(),
            game_systems_to_seed: Vec::new(),
        }
    }

    pub fn with_user(mut self, identifier: &str) -> Self {
        let unique_id = Uuid::new_v4();
        self.users_to_seed.push(UserSeedOptions {
            identifier: identifier.to_string(),
            username: format!("{}-{}", identifier, unique_id),
            email: format!("{}@test.com", unique_id),
        });
        self
    }

    pub fn with_table(mut self, identifier: &str, owner_identifier: &str) -> Self {
        self.tables_to_seed.push(TableSeedOptions {
            identifier: identifier.to_string(),
            title: format!("Table {}", identifier),
            owner_identifier: owner_identifier.to_string(),
        });
        self
    }

    pub fn with_session(mut self, identifier: &str, table_identifier: &str) -> Self {
        self.sessions_to_seed.push(SessionSeedOptions {
            identifier: identifier.to_string(),
            name: format!("Session {}", identifier),
            table_identifier: table_identifier.to_string(),
        });
        self
    }

    pub fn with_game_system(mut self, identifier: &str, name: &str) -> Self {
        self.game_systems_to_seed.push(GameSystemSeedOptions {
            identifier: identifier.to_string(),
            name: name.to_string(),
        });
        self
    }

    pub async fn build(self) -> TestEnvironment {
        let mock_server = MockServer::start().await;
        let config = config_for_test(&self.pool);

        let user_repo = Arc::new(PostgresUserRepository::new(self.pool.clone()));
        let user_service = UserService::new(user_repo.clone());

        let password_repo = Arc::new(BcryptPasswordProvider);
        let password_service = PasswordService::new(password_repo.clone());

        let table_repo = Arc::new(PostgresTableRepository::new(self.pool.clone()));
        let table_service = TableService::new(table_repo.clone());

        let table_member_repo = Arc::new(PostgresTableMemberRepository::new(self.pool.clone()));
        let table_member_service = Arc::new(TableMemberService::new(table_member_repo.clone()));

        let table_request_repo = Arc::new(PostgresTableRequestRepository::new(self.pool.clone()));
        let table_request_service = TableRequestService::new(
            table_request_repo.clone(),
            table_repo.clone(),
            table_member_repo.clone(),
            table_member_service.clone(),
        );

        let session_repo = Arc::new(PostgresSessionRepository::new(self.pool.clone()));
        let session_service = SessionService::new(session_repo.clone(), table_repo.clone());

        let search_service = SearchService::new(user_repo.clone(), table_repo.clone());

        let jwt_provider = Arc::new(JwtTokenProvider::new(
            config.jwt_secret.clone(),
            config.jwt_expiration_duration,
        ));
        let refresh_token_repo = Arc::new(PostgresRefreshTokenRepository::new(self.pool.clone()));
        let auth_service = AuthService::new(
            user_repo.clone(),
            password_repo.clone(),
            jwt_provider.clone(),
            refresh_token_repo.clone(),
        );

        let game_system_repo = Arc::new(PostgresGameSystemRepository::new(self.pool.clone()));
        let game_system_service = GameSystemService::new(game_system_repo);

        // Table member service
        let table_member_repo = Arc::new(PostgresTableMemberRepository::new(self.pool.clone()));
        let table_member_service = TableMemberService::new(table_member_repo);

        let state = Arc::new(AppState {
            config: config.clone(),
            user_service: user_service.clone(),
            table_service: table_service.clone(),
            table_request_service,
            session_service: session_service.clone(),
            search_service,
            auth_service: auth_service.clone(),
            password_service,
            game_system_service: game_system_service.clone(),
            table_member_service: table_member_service.clone(), // Add this line
        });

        let server = TestServer::new(jos::api::http::handlers::create_router(state.clone()))
            .expect("Failed to create test server");

        let mut seeded = SeededData::default();

        // Seed GameSystem
        for gs_opts in self.game_systems_to_seed {
            let mut gs_cmd = CreateGameSystemCommand { name: gs_opts.name };
            let gs = game_system_service.create(&mut gs_cmd).await.unwrap();
            seeded.game_systems.insert(gs_opts.identifier, gs);
        }

        let mut gs_cmd = CreateGameSystemCommand {
            name: "Default Test GS".to_string(),
        };
        let default_gs = game_system_service.create(&mut gs_cmd).await.unwrap();
        seeded
            .game_systems
            .insert("default".to_string(), default_gs.clone());

        // Seed Users
        for user_opts in self.users_to_seed {
            let mut cmd = CreateUserCommand {
                username: user_opts.username,
                email: user_opts.email,
                password: DEFAULT_USER_PASSWORD.to_string(),
            };
            // Use the auth_service to register the user, which handles password hashing
            let user = auth_service.register(&mut cmd).await.unwrap();
            seeded.users.insert(user_opts.identifier, user);
        }

        // Seed Tables
        for table_opts in &self.tables_to_seed {
            let owner = seeded.users.get(&table_opts.owner_identifier).unwrap();
            let cmd = CreateTableCommand {
                title: table_opts.title.clone(),
                description: "A test table".to_string(),
                gm_id: owner.id,
                slots: 5,
                game_system_id: default_gs.id,
            };
            let table = table_service.create(&cmd).await.unwrap();
            seeded.tables.insert(table_opts.identifier.clone(), table);
        }

        // Seed Sessions
        for session_opts in self.sessions_to_seed {
            let table = seeded.tables.get(&session_opts.table_identifier).unwrap();
            let table_seed_opts = self
                .tables_to_seed
                .iter()
                .find(|t| t.identifier == session_opts.table_identifier)
                .unwrap();
            let owner = seeded.users.get(&table_seed_opts.owner_identifier).unwrap();
            let cmd = CreateSessionCommand {
                table_id: table.id,
                name: session_opts.name,
                description: "A test session".to_string(),
                scheduled_for: None,
                status: SessionStatus::Scheduled,
            };
            let session = session_service.create(owner.id, cmd).await.unwrap();
            seeded.sessions.insert(session_opts.identifier, session);
        }

        TestEnvironment {
            server,
            state,
            seeded,
            mock_server,
        }
    }
}

pub async fn register_and_login(server: &TestServer, email: &str, password: &str) -> String {
    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    login_response.assert_status_ok();
    let login_json = login_response.json::<LoginResponse>();
    login_json.token
}
