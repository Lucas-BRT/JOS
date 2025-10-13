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
async fn test_register_and_login() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Test registration
    let response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    // Test login
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_me() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Test registration
    let response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    // Test login
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let json_body = response.json::<serde_json::Value>();
    let token = json_body["token"].as_str().unwrap();

    // Test me
    let response = server
        .get("/v1/auth/me")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);

    let user = response.json::<UserResponse>();

    assert_eq!(user.email, email);
    assert_eq!(user.username, username);
}

#[tokio::test]
async fn test_logout() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Test registration
    let response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    // Test login
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let json_body = response.json::<serde_json::Value>();
    let token = json_body["token"].as_str().unwrap();

    // Test logout
    let response = server
        .post("/v1/auth/logout")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_refresh() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Test registration
    let response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::CREATED);

    // Test login
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let json_body = response.json::<LoginResponse>();
    let refresh_token = json_body.refresh_token.as_str();
    let token = json_body.token.as_str();

    // Test refresh
    let response = server
        .post("/v1/auth/refresh")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "refresh_token": refresh_token
        }))
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_login_wrong_password() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Register user
    server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email.clone(),
            "password": password
        }))
        .await
        .assert_status(StatusCode::CREATED);

    // Attempt login with wrong password
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": "WrongPassword!"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_non_existent_user() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    // Attempt login with a non-existent email
    let response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "nonexistent@example.com",
            "password": "Password123!"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_me_no_token() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    // Attempt to access /me without a token
    let response = server.get("/v1/auth/me").await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_me_invalid_token() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    // Attempt to access /me with an invalid token
    let response = server
        .get("/v1/auth/me")
        .add_header("Authorization", "Bearer invalidtoken")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_refresh_after_logout() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    // Register user
    server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email.clone(),
            "password": password
        }))
        .await
        .assert_status(StatusCode::CREATED);

    // Login to get tokens
    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    login_response.assert_status(StatusCode::OK);
    let json_body = login_response.json::<LoginResponse>();
    let refresh_token = json_body.refresh_token.as_str();
    let token = json_body.token.as_str();

    // Logout
    server
        .post("/v1/auth/logout")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await
        .assert_status(StatusCode::OK);

    // Attempt to refresh with the now-revoked refresh token
    let response = server
        .post("/v1/auth/refresh")
        .json(&json!({
            "refresh_token": refresh_token
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}
