use api::http::dtos::LoginResponse;
use axum::http::StatusCode;
use axum_test::TestServer;
use jos::api::http::handlers::create_router;
use jos::infrastructure::{config::AppConfig, state::setup_app_state};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use wiremock::MockServer;

pub async fn setup_test_environment(db: &PgPool) -> (TestServer, MockServer) {
    dotenvy::dotenv().ok();
    let mock_server = MockServer::start().await;

    let config = AppConfig::default();

    let app_state = setup_app_state(&db, &config)
        .await
        .expect("failed to setup app state");

    let app_state_arc = Arc::new(app_state);
    let server = create_router(app_state_arc.clone());
    let test_server = TestServer::new(server).unwrap();

    (test_server, mock_server)
}

pub async fn register_and_login(server: &TestServer) -> String {
    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    let login_json = login_response.json::<LoginResponse>();
    login_json.token
}

pub struct JwtAndRefresh {
    pub jwt: String,
    pub refresh: String,
}

pub async fn register_and_login_with_refresh(server: &TestServer) -> JwtAndRefresh {
    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .await;

    let login_json = login_response.json::<LoginResponse>();
    JwtAndRefresh {
        jwt: login_json.token,
        refresh: login_json.refresh_token,
    }
}

pub async fn register_user_and_get_token(server: &TestServer) -> (String, String, String) {
    let username = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", username);
    let password = "Password123!";

    let register_response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": username,
            "email": email.clone(),
            "password": password
        }))
        .await;
    register_response.assert_status(StatusCode::CREATED);

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email.clone(),
            "password": password
        }))
        .await;
    login_response.assert_status_ok();

    let login_json = login_response.json::<LoginResponse>();
    (login_json.token, email, password.to_string())
}
