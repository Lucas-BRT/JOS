use super::utils;
use jos::domain::entities::commands::*;
use jos::domain::entities::update::Update;
use jos::domain::repositories::TableRepository;
use jos::infrastructure::persistence::postgres::repositories::PostgresTableRepository;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_table_success(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let result = repo.create(&table_data).await;

    match result {
        Ok(table) => {
            assert_eq!(table.gm_id, setup.user.id);
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
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let created_table = repo.create(&table_data).await.unwrap();
    let get_command = GetTableCommand {
        id: Some(created_table.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();

    assert_eq!(found_tables.len(), 1);
    let found_table = &found_tables[0];
    assert_eq!(found_table.id, created_table.id);
    assert_eq!(found_table.title, "Test Table");
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
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let user2 = utils::create_user(&pool).await;

    let table_data1 = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Table 1".to_string(),
        description: "First table".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table_data2 = CreateTableCommand {
        gm_id: user2.id,
        title: "Table 2".to_string(),
        description: "Second table".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    repo.create(&table_data1).await.unwrap();
    repo.create(&table_data2).await.unwrap();

    let get_command = GetTableCommand {
        gm_id: Some(setup.user.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables.len(), 2);
    assert!(found_tables.iter().any(|t| t.title == "Table 1"));
    assert!(found_tables.iter().any(|t| t.title == "Test Table"));
}

#[sqlx::test]
async fn test_get_all_tables(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let user2 = utils::create_user(&pool).await;

    let table_data1 = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Table 1".to_string(),
        description: "First table".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table_data2 = CreateTableCommand {
        gm_id: user2.id,
        title: "Table 2".to_string(),
        description: "Second table".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    repo.create(&table_data1).await.unwrap();
    repo.create(&table_data2).await.unwrap();

    let get_command = GetTableCommand::default();
    let all_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(all_tables.len(), 3);
}

#[sqlx::test]
async fn test_update_table_title(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Original Title".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Change("New Title".to_string()),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let get_command = GetTableCommand {
        id: Some(created_table.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables.len(), 1);
    let updated_table = &found_tables[0];
    assert_eq!(updated_table.title, "New Title");
}

#[sqlx::test]
async fn test_update_table_description(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "Original Description".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        description: Update::Change("New Description".to_string()),
        ..Default::default()
    };

    repo.update(&update_data)
        .await
        .expect("Failed to update table");

    let get_command = GetTableCommand {
        id: Some(created_table.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables.len(), 1);
    let updated_table = &found_tables[0];
    assert_eq!(updated_table.description, "New Description");
}

#[sqlx::test]
async fn test_update_table_slots(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        slots: Update::Change(10),
        ..Default::default()
    };

    repo.update(&update_data)
        .await
        .expect("Failed to update table");

    let get_command = GetTableCommand {
        id: Some(created_table.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert_eq!(found_tables.len(), 1);
    let updated_table = &found_tables[0];
    assert_eq!(updated_table.player_slots, 10);
}

#[sqlx::test]
async fn test_delete_table(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };

    let created_table = repo
        .create(&table_data)
        .await
        .expect("Failed to create table");

    let delete_command = DeleteTableCommand {
        id: created_table.id,
        gm_id: setup.user.id,
    };

    let result = repo.delete(&delete_command).await;
    assert!(result.is_ok());

    let get_command = GetTableCommand {
        id: Some(created_table.id),
        ..Default::default()
    };
    let found_tables = repo.read(&get_command).await.unwrap();
    assert!(found_tables.is_empty());
}

#[sqlx::test]
async fn test_delete_table_not_found(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let repo = PostgresTableRepository::new(pool);

    let random_id = Uuid::new_v4();
    let delete_command = DeleteTableCommand {
        id: random_id,
        gm_id: setup.user.id,
    };
    let result = repo.delete(&delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_operations(pool: PgPool) {
    let repo = PostgresTableRepository::new(pool.clone());

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                let user = utils::create_user(&pool).await;
                let game_system = utils::create_game_system(&pool).await;
                let table_data = CreateTableCommand {
                    gm_id: user.id,
                    title: "Concurrent Table".to_string(),
                    description: "A table for concurrency testing".to_string(),
                    slots: 5,
                    game_system_id: game_system.id,
                };
                let repo = PostgresTableRepository::new(pool.clone());
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