#[path = "../utils/mod.rs"]
mod utils;

use api::http::dtos::{LoginResponse, UserResponse};
use axum::http::StatusCode;
use axum_test::TestServer;
use jos::api::http::handlers::create_router;
use jos::infrastructure::{
    config::AppConfig, setup::database::setup_database, state::setup_app_state,
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use wiremock::MockServer;

async fn setup_test_environment() -> (TestServer, PgPool, MockServer) {
    dotenvy::dotenv().ok();
    let mock_server = MockServer::start().await;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = setup_database(&database_url)
        .await
        .expect("failed to setup database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("failed to run migrations");

    let config = AppConfig {
        database_url,
        ..Default::default()
    };

    let app_state = setup_app_state(&db, &config)
        .await
        .expect("failed to setup app state");

    let app_state_arc = Arc::new(app_state);
    let server = create_router(app_state_arc.clone());
    let test_server = TestServer::new(server).unwrap();

    (test_server, db, mock_server)
}

#[tokio::test]
async fn test_update_profile_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // 1. Register and Login
    server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await
        .assert_status(StatusCode::CREATED);

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    login_response.assert_status(StatusCode::OK);
    let login_json = login_response.json::<LoginResponse>();
    let token = login_json.token.as_str();

    // 2. Update Profile
    let new_username = Uuid::new_v4().to_string();
    let new_email = format!("{}@example.com", new_username);
    let update_response = server
        .put("/v1/user/profile")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "username": new_username,
            "email": new_email
        }))
        .await;

    update_response.assert_status(StatusCode::OK);

    // 3. Verify the change was persisted by calling /me
    let me_response = server
        .get("/v1/auth/me")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    me_response.assert_status(StatusCode::OK);
    let user_json = me_response.json::<UserResponse>();

    // This assertion will fail until the logic is implemented
    assert_eq!(user_json.username, new_username);
    assert_eq!(user_json.email, new_email);
}
