use jos::domain::entities::*;
use jos::domain::repositories::*;
use jos::infrastructure::persistence::postgres::repositories::*;
use sqlx::PgPool;

pub struct TestEnvironment {
    pub db_pool: PgPool,
    pub user: User,
    pub game_system: GameSystem,
    pub table: Table,
    pub session: Session,
}

pub async fn setup_test_environment(pool: &PgPool) -> TestEnvironment {
    let user = create_user(pool).await;
    let game_system = create_game_system(pool).await;
    let table = create_table(pool, user.id, game_system.id).await;
    let session = create_session(pool, table.id).await;

    TestEnvironment {
        db_pool: pool.clone(),
        user,
        game_system,
        table,
        session,
    }
}

pub async fn create_user(pool: &PgPool) -> User {
    let repo = PostgresUserRepository::new(pool.clone());
    let unique_id = uuid::Uuid::new_v4().to_string();
    let mut user_data = CreateUserCommand {
        username: format!("testuser-{}", unique_id),
        email: format!("test-{}@example.com", unique_id),
        password: "password123".to_string(),
    };
    repo.create(&mut user_data).await.unwrap()
}

pub async fn create_game_system(pool: &PgPool) -> GameSystem {
    let repo = PostgresGameSystemRepository::new(pool.clone());
    let unique_id = uuid::Uuid::new_v4().to_string();
    let mut game_system_command = CreateGameSystemCommand {
        name: format!("D&D 5e - {}", unique_id),
    };
    repo.create(&mut game_system_command).await.unwrap()
}

pub async fn create_table(pool: &PgPool, user_id: uuid::Uuid, game_system_id: uuid::Uuid) -> Table {
    let repo = PostgresTableRepository::new(pool.clone());
    let table_data = CreateTableCommand {
        gm_id: user_id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id,
    };
    repo.create(&table_data).await.unwrap()
}

pub async fn create_session(pool: &PgPool, table_id: uuid::Uuid) -> Session {
    let repo = PostgresSessionRepository::new(pool.clone());
    let session_data = CreateSessionCommand {
        table_id,
        name: "Test Session".to_string(),
        description: "A session for testing".to_string(),
        scheduled_for: None,
        status: SessionStatus::Scheduled,
    };
    repo.create(session_data).await.unwrap()
}

pub async fn create_session_intent(
    pool: &PgPool,
    user_id: uuid::Uuid,
    session_id: uuid::Uuid,
) -> SessionIntent {
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let session_intent_data = CreateSessionIntentCommand {
        player_id: user_id,
        session_id,
        status: jos::domain::entities::session_intent::IntentStatus::Confirmed,
    };
    repo.create(session_intent_data).await.unwrap()
}
