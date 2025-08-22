use jos::{
    domain::{
        game_system::{GameSystem, GameSystemRepository},
        session::{CreateSessionCommand, Session, SessionRepository},
        session_intent::{
            CreateSessionIntentCommand, IntentStatus, SessionIntent, SessionIntentRepository,
        },
        table::{
            commands::CreateTableCommand,
            entity::{Table, Visibility},
            table_repository::TableRepository,
        },
        user::{User, UserRepository, commands::CreateUserCommand},
    },
    infrastructure::{
        prelude::{
            PostgresGameSystemRepository, PostgresSessionRepository, PostgresTableRepository,
            PostgresUserRepository,
        },
        repositories::session_intent::PostgresSessionIntentRepository,
    },
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool) -> User {
    let repo = PostgresUserRepository::new(Arc::new(pool.clone()));

    let username = format!("testuser{}", Uuid::new_v4());
    let display_name = format!("Test User {}", Uuid::new_v4());
    let email = format!("testuser{}@example.com", Uuid::new_v4());

    let user_data = CreateUserCommand::new(
        username.clone(),
        display_name.clone(),
        email.clone(),
        "password123".to_string(),
    );

    repo.create(&user_data).await.unwrap()
}

pub async fn create_game_system(pool: &PgPool) -> GameSystem {
    let repo = PostgresGameSystemRepository::new(Arc::new(pool.clone()));
    let game_system_name = format!("Test Game System {}", Uuid::new_v4());

    repo.create(&game_system_name).await.unwrap()
}

#[allow(unused)]
pub async fn create_table(pool: &PgPool, gm: User, game_system: GameSystem) -> Table {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));

    let table_name = format!("Test Table {}", Uuid::new_v4());

    let table_data = CreateTableCommand::new(
        gm.id,
        table_name,
        "Test Table Description".to_string(),
        Visibility::Public,
        5,
        game_system.id,
    );

    repo.create(&table_data).await.unwrap()
}

pub async fn create_session(pool: &PgPool, table: Table) -> Session {
    let repo = PostgresSessionRepository::new(Arc::new(pool.clone()));

    let session_name = format!("Test Session {}", Uuid::new_v4());

    let session_data = CreateSessionCommand::new(
        table.id,
        session_name,
        "Test Session Description".to_string(),
        true,
    );

    repo.create(&session_data).await.unwrap()
}

#[allow(unused)]
pub async fn create_session_intent(
    pool: &PgPool,
    user: User,
    session: Session,
    status: IntentStatus,
) -> SessionIntent {
    let repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let session_intent_data = CreateSessionIntentCommand::new(user.id, session.id, status);

    repo.create(session_intent_data).await.unwrap()
}
