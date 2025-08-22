mod utils;

use jos::Error;
use jos::domain::table_request::{
    dtos::{
        CreateTableRequestCommand, DeleteTableRequestCommand, GetTableRequestCommand,
        TableRequestFilters, UpdateTableRequestCommand,
    },
    entity::TableRequestStatus,
    table_request_repository::TableRequestRepository as TableRequestRepositoryTrait,
};
use jos::domain::utils::pagination::Pagination;
use jos::domain::utils::update::Update;
use jos::infrastructure::repositories::error::RepositoryError;
use jos::infrastructure::repositories::table_request::PostgresTableRequestRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_table_request_success(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let user = utils::create_user(&pool).await;
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table = utils::create_table(&pool, gm, game_system).await;
    let message = Some("I would like to join this table".to_string());

    let request_data = CreateTableRequestCommand::new(user.id, table.id, message.clone());

    let result = repo.create(&request_data).await;

    match result {
        Ok(table_request) => {
            assert_eq!(table_request.user_id, user.id);
            assert_eq!(table_request.table_id, table.id);
            assert_eq!(table_request.message, message);
            assert_eq!(table_request.status, TableRequestStatus::Pending);
            assert!(table_request.id != Uuid::nil());
            assert!(table_request.created_at <= chrono::Utc::now());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_table_request_without_message(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let user = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let gm = utils::create_user(&pool).await;

    let table = utils::create_table(&pool, gm, game_system).await;

    let request_data = CreateTableRequestCommand::new(user.id, table.id, None);

    let result = repo.create(&request_data).await;

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
async fn test_create_multiple_table_requests_success(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let user1 = utils::create_user(&pool).await;
    let user2 = utils::create_user(&pool).await;

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm, game_system).await;

    let request_data1 =
        CreateTableRequestCommand::new(user1.id, table.id, Some("First request".to_string()));
    let request_data2 =
        CreateTableRequestCommand::new(user2.id, table.id, Some("Second request".to_string()));

    let result1 = repo.create(&request_data1).await;
    let result2 = repo.create(&request_data2).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    let table_request1 = result1.unwrap();
    let table_request2 = result2.unwrap();

    assert_eq!(table_request1.user_id, user1.id);
    assert_eq!(table_request2.user_id, user2.id);
    assert_eq!(table_request1.table_id, table.id);
    assert_eq!(table_request2.table_id, table.id);
    assert_ne!(table_request1.id, table_request2.id);
}

#[sqlx::test]
async fn test_delete_table_request_success(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let user = utils::create_user(&pool).await;

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm, game_system).await;
    let gm = utils::create_user(&pool).await;

    let request_data =
        CreateTableRequestCommand::new(user.id, table.id, Some("Test request".to_string()));

    let created_request = repo.create(&request_data).await.unwrap();
    let request_id = created_request.id;

    let delete_data = DeleteTableRequestCommand {
        id: request_id,
        gm_id: gm.id,
    };

    let result = repo.delete(&delete_data).await;

    match result {
        Ok(deleted_request) => {
            assert_eq!(deleted_request.id, request_id);
            assert_eq!(deleted_request.user_id, user.id);
            assert_eq!(deleted_request.table_id, table.id);
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_delete_table_request_not_found(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let random_id = Uuid::new_v4();
    let gm = utils::create_user(&pool).await;

    let delete_data = DeleteTableRequestCommand {
        id: random_id,
        gm_id: gm.id,
    };

    let result = repo.delete(&delete_data).await;

    match result {
        Err(Error::Repository(RepositoryError::TableRequestNotFound)) => (),
        _ => panic!("Expected TableRequestNotFound error, got: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_request_operations(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let handles: Vec<_> = (0..5)
        .map(async |i| {
            let repo = repo.clone();
            let user = utils::create_user(&pool).await;

            let gm = utils::create_user(&pool).await;
            let game_system = utils::create_game_system(&pool).await;
            let table = utils::create_table(&pool, gm, game_system).await;

            let message = Some(format!("Request {}", i));
            let request_data = CreateTableRequestCommand::new(user.id, table.id, message);
            repo.create(&request_data).await
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }
}

#[sqlx::test]
async fn test_update_table_request(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));
    let player = utils::create_user(&pool).await;

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm, game_system).await;

    let request_data =
        CreateTableRequestCommand::new(player.id, table.id, Some("Initial message".to_string()));

    let created_request = repo.create(&request_data).await.unwrap();

    let update_data = UpdateTableRequestCommand {
        id: created_request.id,
        message: Update::Change(Some("Updated message".to_string())),
        status: Update::Keep,
    };

    let result = repo.update(&update_data).await;

    match result {
        Ok(updated_request) => {
            assert_eq!(updated_request.id, created_request.id);
            assert_eq!(updated_request.message, Some("Updated message".to_string()));
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }

    let found_request = repo.find_by_id(&created_request.id).await.unwrap();
    assert_eq!(found_request.message, Some("Updated message".to_string()));
}

#[sqlx::test]
async fn test_get_table_requests(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let filters = TableRequestFilters::default();
    let pagination = Pagination::default();

    let result = repo
        .get(&GetTableRequestCommand {
            filters,
            pagination,
        })
        .await;

    assert!(result.is_ok());
}

#[sqlx::test]
async fn test_find_table_request(pool: PgPool) {
    let repo = PostgresTableRequestRepository::new(Arc::new(pool.clone()));

    let user = utils::create_user(&pool).await;

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm, game_system).await;

    let table_request_command = CreateTableRequestCommand::new(user.id, table.id, None);

    let table_request = repo.create(&table_request_command).await.unwrap();

    let found_table_request = repo.find_by_id(&table_request.id).await.unwrap();

    assert_eq!(found_table_request.id, table_request.id);
}
