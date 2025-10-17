use crate::utils::TestEnvironmentBuilder;
use jos::domain::entities::commands::*;
use jos::domain::entities::update::Update;
use jos::domain::repositories::{TableRepository, UserRepository};
use jos::infrastructure::persistence::postgres::repositories::{PostgresTableRepository, PostgresUserRepository};
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

const GM_ID: &str = "gm";
const OTHER_USER_ID: &str = "other_user";

#[sqlx::test]
async fn test_create_table_success(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());

    let gm = env.seeded.users.get(GM_ID).unwrap();
    let game_system = env.seeded.game_systems.get("default").unwrap();

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };

    let result = repo.create(&table_data).await;

    match result {
        Ok(table) => {
            assert_eq!(table.gm_id, gm.id);
            assert_eq!(table.title, "Test Table");
            assert_eq!(table.description, "A table for testing");
            assert_eq!(table.player_slots, 5);
            assert!(table.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table("table1", GM_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table1 = env.seeded.tables.get("table1").unwrap();

    let get_command = GetTableCommand {
        id: Some(table1.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();

    assert_eq!(found_tables.len(), 1);
    let found_table = &found_tables[0];
    assert_eq!(found_table.id, table1.id);
    assert_eq!(found_table.title, table1.title);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresTableRepository::new(pool);

    let random_id = Uuid::new_v4();
    let get_command = GetTableCommand {
        id: Some(random_id),
        ..Default::default()
    };
    let result = repo.read(&get_command).await;

    assert!(result.is_ok());
    let found_tables = result.unwrap();
    assert!(found_tables.is_empty());
}

#[sqlx::test]
async fn test_find_by_gm_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(OTHER_USER_ID)
        .with_table("table1", GM_ID)
        .with_table("table2", OTHER_USER_ID)
        .with_table("table3", GM_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());

    let gm = env.seeded.users.get(GM_ID).unwrap();

    let get_command = GetTableCommand {
        gm_id: Some(gm.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables.len(), 2);
}

#[sqlx::test]
async fn test_get_all_tables(pool: PgPool) {
    let _env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(OTHER_USER_ID)
        .with_table("table1", GM_ID)
        .with_table("table2", OTHER_USER_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());

    let get_command = GetTableCommand::default();
    let all_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(all_tables.len(), 2);
}

#[sqlx::test]
async fn test_update_table_title(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table("table1", GM_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());
    let table1 = env.seeded.tables.get("table1").unwrap();

    let update_data = UpdateTableCommand {
        id: table1.id,
        title: Update::Change("New Title".to_string()),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let get_command = GetTableCommand {
        id: Some(table1.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    let updated_table = &found_tables[0];
    assert_eq!(updated_table.title, "New Title");
}

#[sqlx::test]
async fn test_update_table_description(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_table("table1", GM_ID).build().await;
    let repo = PostgresTableRepository::new(pool.clone());
    let table1 = env.seeded.tables.get("table1").unwrap();

    let update_data = UpdateTableCommand {
        id: table1.id,
        description: Update::Change("New Description".to_string()),
        ..Default::default()
    };

    repo.update(&update_data).await.expect("Failed to update table");

    let get_command = GetTableCommand { id: Some(table1.id), ..Default::default() };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables[0].description, "New Description");
}

#[sqlx::test]
async fn test_update_table_slots(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_table("table1", GM_ID).build().await;
    let repo = PostgresTableRepository::new(pool.clone());
    let table1 = env.seeded.tables.get("table1").unwrap();

    let update_data = UpdateTableCommand {
        id: table1.id,
        slots: Update::Change(10),
        ..Default::default()
    };

    repo.update(&update_data).await.expect("Failed to update table");

    let get_command = GetTableCommand { id: Some(table1.id), ..Default::default() };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables[0].player_slots, 10);
}

#[sqlx::test]
async fn test_delete_table(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table("table1", GM_ID)
        .build()
        .await;
    let repo = PostgresTableRepository::new(pool.clone());
    let table1 = env.seeded.tables.get("table1").unwrap();
    let gm = env.seeded.users.get(GM_ID).unwrap();

    let delete_command = DeleteTableCommand {
        id: table1.id,
        gm_id: gm.id,
    };

    let result = repo.delete(&delete_command).await;
    assert!(result.is_ok());

    let get_command = GetTableCommand {
        id: Some(table1.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert!(found_tables.is_empty());
}

#[sqlx::test]
async fn test_delete_table_not_found(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).build().await;
    let repo = PostgresTableRepository::new(pool.clone());
    let gm = env.seeded.users.get(GM_ID).unwrap();

    let random_id = Uuid::new_v4();
    let delete_command = DeleteTableCommand {
        id: random_id,
        gm_id: gm.id,
    };
    let result = repo.delete(&delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_operations(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).build().await;
    let user_repo = PostgresUserRepository::new(pool.clone());
    let repo = PostgresTableRepository::new(pool.clone());
    let game_system = env.seeded.game_systems.get("default").unwrap();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let user_repo = user_repo.clone();
            let repo = repo.clone();
            let game_system_id = game_system.id;
            tokio::spawn(async move {
                let mut cmd = CreateUserCommand { username: format!("user-{}", i), email: format!("user-{}@test.com", i), password: "password".to_string() };
                let user = user_repo.create(&mut cmd).await.unwrap();
                let table_data = CreateTableCommand {
                    gm_id: user.id,
                    title: "Concurrent Table".to_string(),
                    description: "A table for concurrency testing".to_string(),
                    slots: 5,
                    game_system_id,
                };
                repo.create(&table_data)
                    .await
                    .expect("Failed to create table")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let get_command = GetTableCommand::default();
    let all_tables = repo
        .read(&get_command)
        .await
        .expect("Failed to get all tables");
    assert_eq!(all_tables.len(), 5);
}