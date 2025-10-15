use jos::{domain::{entities::{commands::{CreateGameSystemCommand, CreateSessionCommand, CreateSessionIntentCommand, CreateTableCommand, CreateUserCommand}, game_system::GameSystem, session::Session, session_intent::{IntentStatus, SessionIntent}, table::Table, user::User}, repositories::{GameSystemRepository, SessionIntentRepository, SessionRepository, TableRepository, UserRepository}}, infrastructure::persistence::postgres::repositories::{PostgresGameSystemRepository, PostgresSessionIntentRepository, PostgresSessionRepository, PostgresTableRepository, PostgresUserRepository}};
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn create_user(pool: &PgPool) -> User {
    let repo = PostgresUserRepository::new(pool.clone());

    let username = format!("testuser{}", Uuid::new_v4());
    let email = format!("testuser{}@example.com", Uuid::new_v4());

    let mut user_data = CreateUserCommand {
        username: username.clone(),
        email: email.clone(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data).await.unwrap()
}

#[allow(dead_code)]
pub async fn create_game_system(pool: &PgPool) -> GameSystem {
    let repo = PostgresGameSystemRepository::new(pool.clone());
    let game_system_name = format!("Test Game System {}", Uuid::new_v4());
    let mut game_system_command = CreateGameSystemCommand {
        name: game_system_name,
    };

    repo.create(&mut game_system_command).await.unwrap()
}

#[allow(unused)]
pub async fn create_table(pool: &PgPool, gm: User, game_system: GameSystem) -> Table {
    let repo = PostgresTableRepository::new(pool.clone());

    let table_name = format!("Test Table {}", Uuid::new_v4());

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: table_name,
        description: "Test Table Description".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };

    repo.create(table_data).await.unwrap()
}

#[allow(unused)]
pub async fn create_session(pool: &PgPool, table: Table) -> Session {
    let repo = PostgresSessionRepository::new(pool.clone());

    let session_name = format!("Test Session {}", Uuid::new_v4());

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: session_name,
        description: "Test Session Description".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    repo.create(session_data).await.unwrap()
}

#[allow(unused)]
pub async fn create_session_intent(
    pool: &PgPool,
    user: User,
    session: Session,
    status: IntentStatus,
) -> SessionIntent {
    let repo = PostgresSessionIntentRepository::new(pool.clone());

    let session_intent_data = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session.id,
        status,
    };

    repo.create(session_intent_data).await.unwrap()
}
