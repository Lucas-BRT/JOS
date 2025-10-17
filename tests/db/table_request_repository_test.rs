use crate::utils::TestEnvironmentBuilder;
use jos::domain::entities::commands::*;
use jos::domain::entities::table_request::TableRequestStatus;
use jos::domain::entities::update::Update;
use jos::domain::repositories::*;
use jos::infrastructure::persistence::postgres::repositories::*;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

const GM_ID: &str = "gm";
const PLAYER_ID: &str = "player";
const TABLE_ID: &str = "table1";

#[sqlx::test]
async fn test_create_table_request_success(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());

    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("I would like to join".to_string()),
    };

    let result = table_request_repo.create(request_data).await;

    match result {
        Ok(request) => {
            assert_eq!(request.user_id, player.id);
            assert_eq!(request.table_id, table.id);
            assert_eq!(request.status, TableRequestStatus::Pending);
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_table_request_without_message(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_user(PLAYER_ID).with_table(TABLE_ID, GM_ID).build().await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let request_data = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: None,
    };

    let result = table_request_repo.create(request_data).await;
    assert!(result.is_ok());
    assert!(result.unwrap().message.is_none());
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_request = table_request_repo
        .create(CreateTableRequestCommand {
            user_id: player.id,
            table_id: table.id,
            message: Some("Test message".to_string()),
        })
        .await
        .unwrap();

    let found_requests = table_request_repo
        .read(GetTableRequestCommand {
            id: Some(created_request.id),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(found_requests.len(), 1);
    assert_eq!(found_requests[0].id, created_request.id);
}

#[sqlx::test]
async fn test_find_by_user_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_table("table2", GM_ID)
        .build()
        .await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table1 = env.seeded.tables.get(TABLE_ID).unwrap();
    let table2 = env.seeded.tables.get("table2").unwrap();

    table_request_repo
        .create(CreateTableRequestCommand {
            user_id: player.id,
            table_id: table1.id,
            message: None,
        })
        .await
        .unwrap();
    table_request_repo
        .create(CreateTableRequestCommand {
            user_id: player.id,
            table_id: table2.id,
            message: None,
        })
        .await
        .unwrap();

    let found_requests = table_request_repo
        .read(GetTableRequestCommand {
            user_id: Some(player.id),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(found_requests.len(), 2);
}

#[sqlx::test]
async fn test_get_all_table_requests(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_user(PLAYER_ID).with_user("player2").with_table(TABLE_ID, GM_ID).build().await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player1 = env.seeded.users.get(PLAYER_ID).unwrap();
    let player2 = env.seeded.users.get("player2").unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    table_request_repo.create(CreateTableRequestCommand { user_id: player1.id, table_id: table.id, message: None }).await.unwrap();
    table_request_repo.create(CreateTableRequestCommand { user_id: player2.id, table_id: table.id, message: None }).await.unwrap();

    let all_requests = table_request_repo.read(GetTableRequestCommand::default()).await.unwrap();
    assert_eq!(all_requests.len(), 2);
}

#[sqlx::test]
async fn test_update_table_request_status(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_request = table_request_repo
        .create(CreateTableRequestCommand {
            user_id: player.id,
            table_id: table.id,
            message: None,
        })
        .await
        .unwrap();

    table_request_repo
        .update(UpdateTableRequestCommand {
            id: created_request.id,
            status: Update::Change(TableRequestStatus::Approved),
            ..Default::default()
        })
        .await
        .unwrap();

    let found_requests = table_request_repo
        .read(GetTableRequestCommand {
            id: Some(created_request.id),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(found_requests[0].status, TableRequestStatus::Approved);
}

#[sqlx::test]
async fn test_update_table_request_message(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_user(PLAYER_ID).with_table(TABLE_ID, GM_ID).build().await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_request = table_request_repo.create(CreateTableRequestCommand { user_id: player.id, table_id: table.id, message: Some("Original".to_string()) }).await.unwrap();

    table_request_repo.update(UpdateTableRequestCommand { id: created_request.id, message: Update::Change(Some("New".to_string())), ..Default::default() }).await.unwrap();

    let found_requests = table_request_repo.read(GetTableRequestCommand { id: Some(created_request.id), ..Default::default() }).await.unwrap();
    assert_eq!(found_requests[0].message, Some("New".to_string()));
}

#[sqlx::test]
async fn test_delete_table_request(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let table_request_repo = PostgresTableRequestRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_request = table_request_repo
        .create(CreateTableRequestCommand {
            user_id: player.id,
            table_id: table.id,
            message: None,
        })
        .await
        .unwrap();

    let deleted_request = table_request_repo
        .delete(DeleteTableRequestCommand { id: created_request.id })
        .await
        .unwrap();

    assert_eq!(deleted_request.id, created_request.id);

    let found = table_request_repo
        .read(GetTableRequestCommand {
            id: Some(created_request.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(found.is_empty());
}

#[sqlx::test]
async fn test_delete_table_request_not_found(pool: PgPool) {
    let table_request_repo = PostgresTableRequestRepository::new(pool);
    let random_id = Uuid::new_v4();
    let result = table_request_repo
        .delete(DeleteTableRequestCommand { id: random_id })
        .await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_table_request_operations(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_table(TABLE_ID, GM_ID).build().await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let user_repo = PostgresUserRepository::new(pool.clone());
    let repo = PostgresTableRequestRepository::new(pool.clone());

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let user_repo = user_repo.clone();
            let repo = repo.clone();
            let table_id = table.id;
            tokio::spawn(async move {
                let mut cmd = CreateUserCommand { username: format!("player-{}", i), email: format!("player-{}@test.com", i), password: "password".to_string() };
                let user = user_repo.create(&mut cmd).await.unwrap();
                let request_data = CreateTableRequestCommand {
                    user_id: user.id,
                    table_id,
                    message: Some("Concurrent request".to_string()),
                };
                repo
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
    let all_requests = repo
        .read(get_command)
        .await
        .expect("Failed to get all table requests");
    assert_eq!(all_requests.len(), 5);
}