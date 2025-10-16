use super::utils;
use jos::domain::entities::commands::*;
use jos::domain::entities::table_request::TableRequestStatus;
use jos::domain::entities::update::Update;
use jos::domain::repositories::*;
use jos::infrastructure::persistence::postgres::repositories::*;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_table_request_success(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: Some("I would like to join".to_string()),
    };

    let result = table_request_repo.create(request_data).await;

    match result {
        Ok(request) => {
            assert_eq!(request.user_id, setup.user.id);
            assert_eq!(request.table_id, setup.table.id);
            assert_eq!(request.message, Some("I would like to join".to_string()));
            assert_eq!(request.status, TableRequestStatus::Pending);
            assert!(request.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_table_request_without_message(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: None,
    };

    let result = table_request_repo.create(request_data).await;

    match result {
        Ok(request) => {
            assert_eq!(request.user_id, setup.user.id);
            assert_eq!(request.table_id, setup.table.id);
            assert!(request.message.is_none());
            assert_eq!(request.status, TableRequestStatus::Pending);
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: Some("Test message".to_string()),
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
    assert_eq!(found_request.message, Some("Test message".to_string()));
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
    let setup = utils::setup_test_environment(&pool).await;
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let table_data2 = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Another Table".to_string(),
        description: "Table for another request".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table2 = table_repo.create(&table_data2).await.unwrap();

    let request_data1 = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: None,
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: table2.id,
        message: None,
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand {
        user_id: Some(setup.user.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 2);
}

#[sqlx::test]
async fn test_find_by_table_id(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user2 = utils::create_user(&pool).await;

    let request_data1 = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: None,
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: user2.id,
        table_id: setup.table.id,
        message: None,
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand {
        table_id: Some(setup.table.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 2);
}

#[sqlx::test]
async fn test_get_all_table_requests(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let user2 = utils::create_user(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table = table_repo.create(&table_data).await.unwrap();

    let request_data1 = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: table.id,
        message: None,
    };
    let request_data2 = CreateTableRequestCommand {
        user_id: user2.id,
        table_id: table.id,
        message: None,
    };

    table_request_repo.create(request_data1).await.unwrap();
    table_request_repo.create(request_data2).await.unwrap();

    let get_command = GetTableRequestCommand::default();
    let all_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(all_requests.len(), 2);
}

#[sqlx::test]
async fn test_update_table_request_status(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: None,
    };

    let created_request = table_request_repo.create(request_data).await.unwrap();

    let update_data = UpdateTableRequestCommand {
        id: created_request.id,
        status: Update::Change(TableRequestStatus::Pending),
        ..Default::default()
    };

    let result = table_request_repo.update(update_data).await;
    assert!(result.is_ok());

    let get_command = GetTableRequestCommand {
        id: Some(created_request.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 1);
    let updated_request = &found_requests[0];
    assert_eq!(updated_request.status, TableRequestStatus::Pending);
}

#[sqlx::test]
async fn test_update_table_request_message(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: setup.table.id,
        message: Some("Original message".to_string()),
    };

    let created_request = table_request_repo.create(request_data).await.unwrap();

    let update_data = UpdateTableRequestCommand {
        id: created_request.id,
        message: Update::Change(Some("New message".to_string())),
        ..Default::default()
    };

    let result = table_request_repo.update(update_data).await;
    assert!(result.is_ok());

    let get_command = GetTableRequestCommand {
        id: Some(created_request.id),
        ..Default::default()
    };
    let found_requests = table_request_repo.read(get_command).await.unwrap();
    assert_eq!(found_requests.len(), 1);
    let updated_request = &found_requests[0];
    assert_eq!(updated_request.message, Some("New message".to_string()));
}

#[sqlx::test]
async fn test_delete_table_request(pool: PgPool) {
    let setup = utils::setup_test_environment(&pool).await;
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table = table_repo.create(&table_data).await.unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: setup.user.id,
        table_id: table.id,
        message: None,
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
    let setup = utils::setup_test_environment(&pool).await;
    let table_repo = PostgresTableRepository::new(pool.clone());
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let table_data = CreateTableCommand {
        gm_id: setup.user.id,
        title: "Test Table".to_string(),
        description: "A table for testing".to_string(),
        slots: 5,
        game_system_id: setup.game_system.id,
    };
    let table = table_repo.create(&table_data).await.unwrap();

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let pool = pool.clone();
            let table_id = table.id;
            tokio::spawn(async move {
                let user = utils::create_user(&pool).await;
                let request_data = CreateTableRequestCommand {
                    user_id: user.id,
                    table_id,
                    message: Some("Concurrent request".to_string()),
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
