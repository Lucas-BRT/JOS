#[path = "./utils/mod.rs"]
mod utils;

use jos::domain::entities::commands::{
    CreateTableCommand, CreateTableRequestCommand, DeleteTableRequestCommand,
    GetTableRequestCommand, UpdateTableRequestCommand,
};
use jos::domain::entities::table_request::TableRequestStatus;
use jos::domain::entities::update::Update;
use jos::domain::repositories::{TableRepository, TableRequestRepository};
use jos::infrastructure::persistence::postgres::repositories::{
    PostgresTableRepository, PostgresTableRequestRepository,
};
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_table_request_success(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let message = Some("I would like to join this table".to_string());
    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: message.clone(),
    };

    let result = table_request_repo.create(request_data).await;

    match result {
        Ok(table_request) => {
            assert_eq!(table_request.user_id, user.id);
            assert_eq!(table_request.table_id, table.id);
            assert_eq!(table_request.message, message);
            assert_eq!(table_request.status, TableRequestStatus::Pending);
            assert!(table_request.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_table_request_without_message(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: None,
    };

    let result = table_request_repo.create(request_data).await;

    match result {
        Ok(table_request) => {
            assert_eq!(table_request.user_id, user.id);
            assert_eq!(table_request.table_id, table.id);
            assert_eq!(table_request.message, None);
            assert_eq!(table_request.status, TableRequestStatus::Pending);
            assert!(table_request.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: Some("I would like to join this table".to_string()),
    };

    let created_request = table_request_repo.create(request_data).await.unwrap();
    let get_command = GetTableRequestCommand {
        id: Some(created_request.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();

    assert_eq!(found_requests.len(), 1);
    let found_request = &found_requests[0];
    assert_eq!(found_request.id, created_request.id);
    assert_eq!(found_request.user_id, user.id);
    assert_eq!(found_request.table_id, table.id);
    assert_eq!(found_request.status, TableRequestStatus::Pending);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let table_request_repo = PostgresTableRequestRepository::new(pool);

    let random_id = Uuid::new_v4();
    let get_command = GetTableRequestCommand {
        id: Some(random_id),
        ..Default::default()
    };
    let result = table_request_repo.read(get_command).await;

    assert!(result.is_ok());
    let found_requests = result.unwrap();
    assert!(found_requests.is_empty());
}

#[sqlx::test]
async fn test_find_by_user_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand {
        gm_id: gm.id,
        title: "Table 1".to_string(),
        description: "First table".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table1 = table_repo.create(table_data1).await.unwrap();

    let table_data2 = CreateTableCommand {
        gm_id: gm.id,
        title: "Table 2".to_string(),
        description: "Second table".to_string(),
        slots: 3,
        game_system_id: game_system.id,
    };
    let table2 = table_repo.create(table_data2).await.unwrap();

    let request_data1 = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table1.id,
        message: Some("Request for table 1".to_string()),
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table2.id,
        message: Some("Request for table 2".to_string()),
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand {
        user_id: Some(user.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 2);

    let table_ids: Vec<Uuid> = found_requests.iter().map(|r| r.table_id).collect();
    assert!(table_ids.contains(&table1.id));
    assert!(table_ids.contains(&table2.id));
}

#[sqlx::test]
async fn test_find_by_table_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user1 = utils::create_user(&pool).await;
    let user2 = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data1 = CreateTableRequestCommand {
        user_id: user1.id,
        table_id: table.id,
        message: Some("Request from user 1".to_string()),
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: user2.id,
        table_id: table.id,
        message: Some("Request from user 2".to_string()),
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand {
        table_id: Some(table.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 2);

    let user_ids: Vec<Uuid> = found_requests.iter().map(|r| r.user_id).collect();
    assert!(user_ids.contains(&user1.id));
    assert!(user_ids.contains(&user2.id));
}

#[sqlx::test]
async fn test_get_all_table_requests(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user1 = utils::create_user(&pool).await;
    let user2 = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data1 = CreateTableRequestCommand {
        user_id: user1.id,
        table_id: table.id,
        message: Some("Request from user 1".to_string()),
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: user2.id,
        table_id: table.id,
        message: Some("Request from user 2".to_string()),
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand::default();
    let all_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(all_requests.len(), 2);

    let user_ids: Vec<Uuid> = all_requests.iter().map(|r| r.user_id).collect();
    assert!(user_ids.contains(&user1.id));
    assert!(user_ids.contains(&user2.id));
}

#[sqlx::test]
async fn test_update_table_request_status(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: Some("I would like to join this table".to_string()),
    };

    let created_request = table_request_repo.create(request_data).await.unwrap();

    let update_data = UpdateTableRequestCommand {
        id: created_request.id,
        status: Update::Change(TableRequestStatus::Approved),
        ..Default::default()
    };

    table_request_repo
        .update(update_data)
        .await
        .expect("Failed to update table request");

    let get_command = GetTableRequestCommand {
        id: Some(created_request.id),
        ..Default::default()
    };

    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 1);

    let updated_request = &found_requests[0];
    assert_eq!(updated_request.status, TableRequestStatus::Approved);
}

#[sqlx::test]
async fn test_update_table_request_message(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: Some("Original message".to_string()),
    };

    let created_request = table_request_repo.create(request_data).await.unwrap();

    let update_data = UpdateTableRequestCommand {
        id: created_request.id,
        message: Update::Change(Some("Updated message".to_string())),
        ..Default::default()
    };

    table_request_repo
        .update(update_data)
        .await
        .expect("Failed to update table request");

    let get_command = GetTableRequestCommand {
        id: Some(created_request.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 1);
    let updated_request = &found_requests[0];
    assert_eq!(updated_request.message, Some("Updated message".to_string()));
}

#[sqlx::test]
async fn test_delete_table_request(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: user.id,
        table_id: table.id,
        message: Some("I would like to join this table".to_string()),
    };

    let created_request = table_request_repo
        .create(request_data)
        .await
        .expect("Failed to create table request");

    let delete_command = DeleteTableRequestCommand {
        id: created_request.id,
    };

    let deleted_request = table_request_repo
        .delete(delete_command)
        .await
        .expect("Failed to delete table request");

    assert_eq!(deleted_request.id, created_request.id);
    assert_eq!(deleted_request.user_id, user.id);
    assert_eq!(deleted_request.table_id, table.id);
}

#[sqlx::test]
async fn test_delete_table_request_not_found(pool: PgPool) {
    let table_request_repo = PostgresTableRequestRepository::new(pool);

    let random_id = Uuid::new_v4();
    let delete_command = DeleteTableRequestCommand { id: random_id };
    let result = table_request_repo.delete(delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_request_operations(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 10,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let pool = pool.clone();
            tokio::spawn(async move {
                let user = utils::create_user(&pool).await;
                let request_data = CreateTableRequestCommand {
                    user_id: user.id,
                    table_id: table.id,
                    message: Some(format!("Request {i}")),
                };
                let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
                table_request_repo
                    .create(request_data)
                    .await
                    .expect("Failed to create table request")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let get_command = GetTableRequestCommand::default();
    let all_requests = table_request_repo
        .read(get_command)
        .await
        .expect("Failed to get all table requests");
    assert_eq!(all_requests.len(), 5);
}
