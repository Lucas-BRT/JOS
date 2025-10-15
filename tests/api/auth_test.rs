use super::utils::*;
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

#[tokio::test]
async fn test_update_profile_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

    // 2. Update Profile
    let new_username = Uuid::new_v4().to_string();
    let new_email = format!("{}@example.com", new_username);
    let update_response = server
        .put("/v1/auth/profile")
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

#[tokio::test]
async fn test_change_password_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let new_password = "NewPassword456!";

    // Register and get token
    let (token, email, current_password) = register_user_and_get_token(&server).await;

    // Change password
    let response = server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "current_password": current_password,
            "new_password": new_password,
            "confirm_password": new_password
        }))
        .await;

    response.assert_status(StatusCode::OK);

    // Try to login with the new password
    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": email,
            "password": new_password
        }))
        .await;

    login_response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_change_password_wrong_current_password() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let new_password = "NewPassword456!";

    // Register and get token
    let (token, _, _) = register_user_and_get_token(&server).await;

    // Attempt to change password with wrong current_password
    let response = server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "current_password": "WrongPassword!",
            "new_password": new_password,
            "confirm_password": new_password
        }))
        .await;

    response.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_change_password_mismatched_new_password() {
    let (server, _pool, _mock_server) = setup_test_environment().await;

    // Register and get token
    let (token, _, current_password) = register_user_and_get_token(&server).await;

    // Attempt to change password with mismatched new_password and confirm_password
    let response = server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "current_password": current_password,
            "new_password": "NewPassword456!",
            "confirm_password": "MismatchedPassword!"
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
}