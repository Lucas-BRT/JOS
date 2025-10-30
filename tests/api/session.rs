use crate::utils::{DEFAULT_USER_PASSWORD, TestEnvironmentBuilder, register_and_login};
use api::http::dtos::{CreateSessionRequest, GetSessionsResponse};
use chrono::Utc;
use jos::application::SessionService;
use jos::domain::entities::SessionStatus;
use jos::domain::entities::commands::CreateSessionCommand;
use jos::infrastructure::persistence::postgres::repositories::{
    PostgresSessionRepository, PostgresTableRepository,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

const GM_ID: &str = "gm";
const TABLE_ID: &str = "table1";

#[sqlx::test]
async fn test_get_sessions_from_a_table_empty(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let user = env.seeded.users.get(GM_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let table_id = table.id;

    let response = env
        .server
        .get(&format!("/v1/tables/{table_id}/sessions"))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    assert!(response.json::<Vec<GetSessionsResponse>>().is_empty())
}

#[sqlx::test]
async fn test_get_sessions_from_a_table(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let user = env.seeded.users.get(GM_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let table_id = table.id;

    let response = env
        .server
        .post(&format!("/v1/tables/{table_id}/sessions"))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!(CreateSessionRequest {
            title: "Test Session".to_string(),
            description: "A session for testing".to_string(),
            scheduled_for: Some(Utc::now()),
            status: Some(SessionStatus::default())
        }))
        .await;

    response.assert_status_ok();

    let response = env
        .server
        .get(&format!("/v1/tables/{table_id}/sessions"))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    let body = response.json::<Vec<GetSessionsResponse>>();

    assert!(!body.is_empty());
    assert!(body.iter().any(|session| session.title == "Test Session"));
}

#[sqlx::test]
async fn test_create_session_success(pool: sqlx::PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let user = env.seeded.users.get(GM_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let c_table_id = table.id;

    let response = env
        .server
        .post(&format!("/v1/tables/{c_table_id}/sessions"))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!(CreateSessionRequest {
            title: "Test Session".to_string(),
            description: "A session for testing".to_string(),
            scheduled_for: Some(Utc::now()),
            status: Some(SessionStatus::default())
        }))
        .await;

    response.assert_status_ok();
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
        title: "Unauthorized Session".to_string(),
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
            assert!(matches!(
                app_error,
                jos::shared::error::ApplicationError::Forbidden
            ));
        }
        _ => panic!("Incorrect error type returned"),
    }
}
