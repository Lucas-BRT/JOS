use super::utils::api::{
    register_and_login, register_and_login_with_refresh, setup_test_environment,
};
use api::http::dtos::UserResponse;
use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_register_and_login() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    register_and_login(&server).await;
}

#[tokio::test]
async fn test_me() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

    // Test me
    let response = server
        .get("/v1/auth/me")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);

    let user = response.json::<UserResponse>();

    assert!(!user.email.is_empty());
    assert!(!user.username.is_empty());
}

#[tokio::test]
async fn test_logout() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

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
    let (token, refresh_token) = register_and_login_with_refresh(&server).await;

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
    let (token, refresh_token) = register_and_login_with_refresh(&server).await;

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
