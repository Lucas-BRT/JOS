mod utils;

use jos::Error;
use jos::domain::table::commands::{
    CreateTableCommand, DeleteTableCommand, GetTableCommand, UpdateTableCommand,
};
use jos::domain::table::entity::Visibility;
use jos::domain::table::search_filters::TableFilters;
use jos::domain::table::table_repository::TableRepository;
use jos::domain::utils::pagination::Pagination;
use jos::domain::utils::update::Update;
use jos::infrastructure::prelude::PostgresTableRepository;
use jos::infrastructure::repositories::error::RepositoryError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_table_success(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let result = repo.create(&table_data).await;

    match result {
        Ok(table) => {
            assert_eq!(table.title, "Test Table");
            assert_eq!(table.description, "A test table for RPG");
            assert_eq!(table.visibility, Visibility::Public);
            assert_eq!(table.player_slots, 5);
            assert_eq!(table.gm_id, gm.id);
            assert_eq!(table.game_system_id, game_system.id);
            assert!(table.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_table_private_visibility(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Private Table".into(),
        "A private test table".into(),
        Visibility::Private,
        3,
        game_system.id,
    );

    let result = repo.create(&table_data).await;

    match result {
        Ok(table) => {
            assert_eq!(table.title, "Private Table");
            assert_eq!(table.visibility, Visibility::Private);
            assert_eq!(table.player_slots, 3);
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();
    let found_table = repo.find_by_id(&created_table.id).await;

    assert!(found_table.is_ok());
    let found_table = found_table.unwrap();
    assert_eq!(found_table.id, created_table.id);
    assert_eq!(found_table.title, "Test Table");
    assert_eq!(found_table.description, "A test table for RPG");
    assert_eq!(found_table.visibility, Visibility::Public);
    assert_eq!(found_table.player_slots, 5);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));

    let random_id = Uuid::new_v4();
    let result = repo.find_by_id(&random_id).await;

    assert!(result.is_err());

    if let Err(Error::Repository(RepositoryError::TableNotFound)) = result {
        // Expected error
    } else {
        panic!("Expected TableNotFound error");
    }
}

#[sqlx::test]
async fn test_find_by_gm_id(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand::new(
        gm.id,
        "Table 1".into(),
        "First table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let table_data2 = CreateTableCommand::new(
        gm.id,
        "Table 2".into(),
        "Second table".into(),
        Visibility::Private,
        3,
        game_system.id,
    );

    repo.create(&table_data1).await.unwrap();
    repo.create(&table_data2).await.unwrap();

    let found_tables = repo.find_by_gm_id(&gm.id).await.unwrap();
    assert_eq!(found_tables.len(), 2);

    let titles: Vec<String> = found_tables.iter().map(|t| t.title.clone()).collect();
    assert!(titles.contains(&"Table 1".to_string()));
    assert!(titles.contains(&"Table 2".to_string()));
}

#[sqlx::test]
async fn test_find_by_gm_id_empty(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool));
    let random_gm_id = Uuid::new_v4();

    let found_tables = repo.find_by_gm_id(&random_gm_id).await.unwrap();
    assert_eq!(found_tables.len(), 0);
}

#[sqlx::test]
async fn test_get_all_tables(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm1 = utils::create_user(&pool).await;
    let gm2 = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand::new(
        gm1.id,
        "Table 1".into(),
        "First table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let table_data2 = CreateTableCommand::new(
        gm2.id,
        "Table 2".into(),
        "Second table".into(),
        Visibility::Private,
        3,
        game_system.id,
    );

    repo.create(&table_data1).await.unwrap();
    repo.create(&table_data2).await.unwrap();

    let get_command = GetTableCommand::default();
    let all_tables = repo.get(&get_command).await.unwrap();
    assert_eq!(all_tables.len(), 2);

    let titles: Vec<String> = all_tables.iter().map(|t| t.title.clone()).collect();
    assert!(titles.contains(&"Table 1".to_string()));
    assert!(titles.contains(&"Table 2".to_string()));
}

#[sqlx::test]
async fn test_get_tables_with_filters(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand::new(
        gm.id,
        "Public Table".into(),
        "Public table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let table_data2 = CreateTableCommand::new(
        gm.id,
        "Private Table".into(),
        "Private table".into(),
        Visibility::Private,
        3,
        game_system.id,
    );

    repo.create(&table_data1).await.unwrap();
    repo.create(&table_data2).await.unwrap();

    let filters = TableFilters::default().with_visibility(Visibility::Public);
    let get_command = GetTableCommand::default().with_filters(filters);
    let public_tables = repo.get(&get_command).await.unwrap();

    assert_eq!(public_tables.len(), 1);
    assert_eq!(public_tables[0].title, "Public Table");
    assert_eq!(public_tables[0].visibility, Visibility::Public);
}

#[sqlx::test]
async fn test_update_table_title(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Original Title".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Change("Updated Title".to_string()),
        description: Update::Keep,
        visibility: Update::Keep,
        player_slots: Update::Keep,
        game_system_id: Update::Keep,
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.title, "Updated Title");
    assert_eq!(updated_table.description, "A test table for RPG");
    assert_eq!(updated_table.visibility, Visibility::Public);
}

#[sqlx::test]
async fn test_update_table_description(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "Original description".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Keep,
        description: Update::Change("Updated description".to_string()),
        visibility: Update::Keep,
        player_slots: Update::Keep,
        game_system_id: Update::Keep,
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.title, "Test Table"); // Not changed
    assert_eq!(updated_table.description, "Updated description");
}

#[sqlx::test]
async fn test_update_table_visibility(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Keep,
        description: Update::Keep,
        visibility: Update::Change(Visibility::Private),
        player_slots: Update::Keep,
        game_system_id: Update::Keep,
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.visibility, Visibility::Private);
}

#[sqlx::test]
async fn test_update_table_player_slots(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Keep,
        description: Update::Keep,
        visibility: Update::Keep,
        player_slots: Update::Change(8),
        game_system_id: Update::Keep,
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.player_slots, 8);
}

#[sqlx::test]
async fn test_update_table_game_system_id(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    let update_data = UpdateTableCommand {
        id: created_table.id,
        game_system_id: Update::Change(game_system.id),
        ..Default::default()
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.game_system_id, game_system.id);
}

#[sqlx::test]
async fn test_update_table_multiple_fields(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Original Title".into(),
        "Original description".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo.create(&table_data).await.unwrap();

    // Fixed: Don't use ..Default::default() as it will override the id field
    let update_data = UpdateTableCommand {
        id: created_table.id,
        title: Update::Change("New Title".to_string()),
        description: Update::Change("New description".to_string()),
        visibility: Update::Change(Visibility::Private),
        player_slots: Update::Change(7),
        game_system_id: Update::Keep,
    };

    repo.update(&update_data).await.unwrap();

    let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
    assert_eq!(updated_table.title, "New Title");
    assert_eq!(updated_table.description, "New description");
    assert_eq!(updated_table.visibility, Visibility::Private);
    assert_eq!(updated_table.player_slots, 7);
}

#[sqlx::test]
async fn test_update_table_not_found(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool));

    let random_id = Uuid::new_v4();
    let update_data = UpdateTableCommand {
        id: random_id,
        title: Update::Change("New Title".to_string()),
        description: Update::Keep,
        visibility: Update::Keep,
        player_slots: Update::Keep,
        game_system_id: Update::Keep,
    };

    let result = repo.update(&update_data).await;

    match result {
        Err(Error::Repository(RepositoryError::NotFound)) => (),
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_delete_table(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let created_table = repo
        .create(&table_data)
        .await
        .expect("Failed to create table");

    let delete_command = DeleteTableCommand::new(created_table.id, created_table.gm_id);

    let deleted_table = repo
        .delete(&delete_command)
        .await
        .expect("Failed to delete table");

    assert_eq!(deleted_table.id, created_table.id);
    assert_eq!(deleted_table.title, "Test Table");
}

#[sqlx::test]
async fn test_delete_table_not_found(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool));

    let random_id = Uuid::new_v4();
    let random_gm_id = Uuid::new_v4();
    let delete_command = DeleteTableCommand {
        id: random_id,
        gm_id: random_gm_id,
    };

    let result = repo.delete(&delete_command).await;

    match result {
        Err(Error::Repository(RepositoryError::TableNotFound)) => (),
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_operations(pool: PgPool) {
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let pool = pool.clone();
            let table_data = CreateTableCommand::new(
                gm.id,
                format!("Table {i}"),
                format!("Description for table {i}"),
                if i % 2 == 0 {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                3 + i as u32,
                game_system.id,
            );
            tokio::spawn(async move {
                let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
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

    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let get_command = GetTableCommand::default();
    let all_tables = repo
        .get(&get_command)
        .await
        .expect("Failed to get all tables");
    assert_eq!(all_tables.len(), 5);
}

#[sqlx::test]
async fn test_pagination(pool: PgPool) {
    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    // Create 25 tables
    for i in 0..25 {
        let table_data = CreateTableCommand::new(
            gm.id,
            format!("Table {i}"),
            format!("Description {i}"),
            Visibility::Public,
            5,
            game_system.id,
        );
        repo.create(&table_data)
            .await
            .expect("Failed to create table");
    }

    // Test first page (default page size is 20)
    let pagination = Pagination::default();
    let get_command = GetTableCommand::default().with_pagination(pagination);
    let first_page = repo
        .get(&get_command)
        .await
        .expect("Failed to get first page");
    assert_eq!(first_page.len(), 20);

    // Test second page
    let pagination = Pagination::default().with_page(2);
    let get_command = GetTableCommand::default().with_pagination(pagination);
    let second_page = repo
        .get(&get_command)
        .await
        .expect("Failed to get second page");
    assert_eq!(second_page.len(), 5);

    // Test custom page size
    let pagination = Pagination::default().with_page_size(10);
    let get_command = GetTableCommand::default().with_pagination(pagination);
    let custom_page = repo
        .get(&get_command)
        .await
        .expect("Failed to get custom page");
    assert_eq!(custom_page.len(), 10);
}
