use jos::{
    domain::{
        game_system::{GameSystem, GameSystemRepository},
        table::{
            commands::CreateTableCommand,
            entity::{Table, Visibility},
            table_repository::TableRepository,
        },
        user::{User, UserRepository, commands::CreateUserCommand},
    },
    infrastructure::prelude::{
        PostgresGameSystemRepository, PostgresTableRepository, PostgresUserRepository,
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
pub async fn create_table(pool: &PgPool) -> Table {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));

    let gm_id = create_user(pool).await;
    let game_system = create_game_system(pool).await;

    let table_name = format!("Test Table {}", Uuid::new_v4());

    let table_data = CreateTableCommand::new(
        gm_id.id,
        table_name,
        "Test Table Description".to_string(),
        Visibility::Public,
        5,
        game_system.id,
    );

    repo.create(&table_data).await.unwrap()
}
