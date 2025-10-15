use api::http::dtos::LoginResponse;
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

pub async fn setup_test_environment() -> (TestServer, PgPool, MockServer) {
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

pub async fn register_and_login_with_refresh(server: &TestServer) -> (String, String) {
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
    (login_json.token, login_json.refresh_token)
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