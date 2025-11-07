use crate::utils::*;
use api::http::dtos::*;
use chrono::Utc;
use jos::domain::entities::SessionStatus;
use serde_json::json;
use sqlx::PgPool;

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
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user("some_other_player")
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;

    let other_user = env.seeded.users.get("some_other_player").unwrap();
    let other_user_token =
        register_and_login(&env.server, &other_user.email, DEFAULT_USER_PASSWORD).await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let table_id = table.id;

    let response = env
        .server
        .post(&format!("/v1/tables/{table_id}/sessions"))
        .add_header("Authorization", &format!("Bearer {}", other_user_token))
        .json(&json!(CreateSessionRequest {
            title: "Test Session".to_string(),
            description: "A session for testing".to_string(),
            scheduled_for: Some(Utc::now()),
            status: Some(SessionStatus::default())
        }))
        .await;

    response.assert_status_unauthorized();
}

#[sqlx::test]
async fn test_delete_session(pool: sqlx::PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session("session", TABLE_ID)
        .build()
        .await;

    let user = env.seeded.users.get(GM_ID).unwrap();
    let user_token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    let session = env.seeded.sessions.get("session").unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let response = env
        .server
        .delete(&format!("/v1/tables/{}/sessions/{}", table.id, session.id))
        .add_header("Authorization", &format!("Bearer {}", user_token))
        .await;

    response.assert_status_ok();

    let response = env
        .server
        .get(&format!("/v1/tables/{}/sessions", table.id))
        .add_header("Authorization", &format!("Bearer {}", user_token))
        .await;

    assert!(response.json::<Vec<GetSessionsResponse>>().is_empty());
}

#[sqlx::test]
async fn test_update_session(pool: sqlx::PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session("session", TABLE_ID)
        .build()
        .await;

    let user = env.seeded.users.get(GM_ID).unwrap();
    let user_token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    let session = env.seeded.sessions.get("session").unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let response = env
        .server
        .put(&format!("/v1/tables/{}/sessions/{}", table.id, session.id))
        .add_header("Authorization", &format!("Bearer {}", user_token))
        .json(&json!({
            "title": "Updated Title",
            "description": "Updated description"
        }))
        .await;

    response.assert_status_ok();

    let body = response.json::<UpdateSessionResponse>();

    assert_eq!(body.title, "Updated Title");
    assert_eq!(body.description, "Updated description");
    assert_eq!(body.scheduled_for, None);
    assert_eq!(body.status, SessionStatus::default());
}
