use crate::utils::TestEnvironmentBuilder;
use jos::application::SessionService;
use jos::domain::entities::commands::CreateSessionCommand;
use jos::domain::entities::SessionStatus;
use jos::domain::repositories::TableRepository;
use jos::infrastructure::persistence::postgres::repositories::{PostgresSessionRepository, PostgresTableRepository};
use std::sync::Arc;

const GM_ID: &str = "gm";
const TABLE_ID: &str = "table1";

#[sqlx::test]
async fn test_create_session_success(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let table_repo = Arc::new(PostgresTableRepository::new(pool.clone()));

    // This will fail to compile until the service is updated
    let session_service = SessionService::new(session_repo.clone(), table_repo.clone());

    let gm = env.seeded.users.get(GM_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let command = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A session for testing".to_string(),
        scheduled_for: None,
        status: SessionStatus::Scheduled,
    };

    // This will also fail to compile until the method signature is updated
    let result = session_service.create(gm.id, command).await;

    // Assert
    assert!(result.is_ok(), "Expected successful creation, but got {:?}", result.err());
    let session = result.unwrap();
    assert_eq!(session.table_id, table.id);
    assert_eq!(session.name, "Test Session");
    assert_eq!(session.status, SessionStatus::Scheduled);
}

#[sqlx::test]
async fn test_create_session_fails_if_not_gm(pool: sqlx::PgPool) {
    // Arrange
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user("some_other_player")
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let session_repo = Arc::new(PostgresSessionRepository::new(pool.clone()));
    let table_repo = Arc::new(PostgresTableRepository::new(pool.clone()));
    let session_service = SessionService::new(session_repo.clone(), table_repo.clone());

    let not_the_gm = env.seeded.users.get("some_other_player").unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let command = CreateSessionCommand {
        table_id: table.id,
        name: "Unauthorized Session".to_string(),
        description: "A session that should not be created".to_string(),
        scheduled_for: None,
        status: SessionStatus::Scheduled,
    };

    // Act
    let result = session_service.create(not_the_gm.id, command).await;

    // Assert
    assert!(result.is_err(), "Expected an error, but got Ok");
    match result.unwrap_err() {
        jos::shared::error::Error::Application(app_error) => {
            assert!(matches!(app_error, jos::shared::error::ApplicationError::Forbidden));
        }
        _ => panic!("Incorrect error type returned"),
    }
}
