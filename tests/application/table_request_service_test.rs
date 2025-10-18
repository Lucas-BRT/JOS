use crate::utils::TestEnvironmentBuilder;
use jos::application::{TableMemberService, TableRequestService};
use jos::domain::entities::commands::{CreateTableMemberCommand, CreateTableRequestCommand};
use jos::domain::entities::TableRequestStatus;
use jos::domain::repositories::{TableRepository, TableRequestRepository};
use jos::infrastructure::persistence::postgres::repositories::{
    PostgresTableMemberRepository, PostgresTableRepository, PostgresTableRequestRepository,
};
use std::sync::Arc;
use jos::shared::error::{DomainError, Error};

const GM_ID: &str = "gm";
const PLAYER_ID: &str = "player";
const TABLE_ID: &str = "table1";

// Helper to build the service with all its dependencies for testing
fn build_service(pool: sqlx::PgPool) -> (TableRequestService, Arc<PostgresTableRequestRepository>) {
    let table_request_repo = Arc::new(PostgresTableRequestRepository::new(pool.clone()));
    let table_repo = Arc::new(PostgresTableRepository::new(pool.clone()));
    let table_member_repo = Arc::new(PostgresTableMemberRepository::new(pool.clone()));
    let table_member_service = Arc::new(TableMemberService::new(table_member_repo.clone()));

    let service = TableRequestService::new(
        table_request_repo.clone(),
        table_repo,
        table_member_repo,
        table_member_service,
    );
    (service, table_request_repo)
}

#[sqlx::test]
async fn test_create_request_success(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let (table_request_service, table_request_repo) = build_service(pool.clone());

    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let command = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("Please let me join".to_string()),
    };

    // Act
    let result = table_request_service.create(command).await;

    // Assert
    assert!(result.is_ok(), "Expected successful creation, but got {:?}", result.err());
    let request = result.unwrap();
    assert_eq!(request.user_id, player.id);
    assert_eq!(request.table_id, table.id);
    assert_eq!(request.status, TableRequestStatus::Pending);

    // Verify it's in the database
    let found_request = table_request_repo.find_by_id(request.id).await.unwrap().unwrap();
    assert_eq!(found_request.id, request.id);
}

#[sqlx::test]
async fn test_create_request_fails_if_user_is_already_member(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let (table_request_service, _) = build_service(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    // Manually add the player to the table
    env.state.table_member_service.create(CreateTableMemberCommand {
        table_id: table.id,
        user_id: player.id,
    }).await.unwrap();

    let command = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("I want to join again".to_string()),
    };

    // Act
    let result = table_request_service.create(command).await;

    // Assert
    assert!(result.is_err(), "Expected an error, but got Ok");
    match result.unwrap_err() {
        Error::Domain(domain_error) => match domain_error {
            DomainError::BusinessRuleViolation { message } => {
                assert!(message.contains("User is already a member of this table"));
            }
            _ => panic!("Incorrect DomainError type"),
        },
        _ => panic!("Incorrect error type"),
    }
}

#[sqlx::test]
async fn test_create_request_fails_if_user_is_gm(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let (table_request_service, _) = build_service(pool.clone());
    let gm = env.seeded.users.get(GM_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let command = CreateTableRequestCommand {
        user_id: gm.id, // The GM is the one making the request
        table_id: table.id,
        message: Some("I want to join my own table?".to_string()),
    };

    // Act
    let result = table_request_service.create(command).await;

    // Assert
    assert!(result.is_err(), "Expected an error, but got Ok");
    match result.unwrap_err() {
        Error::Domain(domain_error) => match domain_error {
            DomainError::BusinessRuleViolation { message } => {
                assert!(message.contains("Game master cannot request to join their own table"));
            }
            _ => panic!("Incorrect DomainError type"),
        },
        _ => panic!("Incorrect error type"),
    }
}

#[sqlx::test]
async fn test_create_request_fails_if_table_is_not_active(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let (table_request_service, _) = build_service(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    // Manually update the table status to Finished
    let mut update_command = jos::domain::entities::commands::UpdateTableCommand::default();
    update_command.id = table.id;
    update_command.status = jos::domain::entities::Update::Change(jos::domain::entities::TableStatus::Finished);
    env.state.table_service.update(&update_command).await.unwrap();

    let command = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("Can I join this finished table?".to_string()),
    };

    // Act
    let result = table_request_service.create(command).await;

    // Assert
    assert!(result.is_err(), "Expected an error, but got Ok");
    match result.unwrap_err() {
        Error::Domain(domain_error) => match domain_error {
            DomainError::BusinessRuleViolation { message } => {
                assert!(message.contains("Table is not accepting new players"));
            }
            _ => panic!("Incorrect DomainError type"),
        },
        _ => panic!("Incorrect error type"),
    }
}

#[sqlx::test]
async fn test_create_request_fails_if_pending_request_exists(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let (table_request_service, _) = build_service(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    // Create an initial request
    let initial_command = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("First request".to_string()),
    };
    table_request_service.create(initial_command).await.unwrap();

    // Attempt to create another one
    let second_command = CreateTableRequestCommand {
        user_id: player.id,
        table_id: table.id,
        message: Some("Second request".to_string()),
    };

    // Act
    let result = table_request_service.create(second_command).await;

    // Assert
    assert!(result.is_err(), "Expected an error, but got Ok");
    match result.unwrap_err() {
        Error::Domain(domain_error) => match domain_error {
            DomainError::BusinessRuleViolation { message } => {
                assert!(message.contains("A pending request for this table already exists"));
            }
            _ => panic!("Incorrect DomainError type"),
        },
        _ => panic!("Incorrect error type"),
    }
}