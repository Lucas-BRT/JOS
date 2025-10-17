use crate::utils::{TestEnvironmentBuilder, register_and_login};
use api::http::dtos::UserResponse;
use axum::http::StatusCode;
use sqlx::PgPool;

const TEST_USER_ID: &str = "test_user";
const TEST_PASSWORD: &str = "Password123!";

#[sqlx::test]
async fn test_register_and_login(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();

    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    assert!(!token.is_empty());
}

#[sqlx::test]
async fn test_me(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get("/v1/auth/me")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let user_res = response.json::<UserResponse>();
    assert_eq!(user_res.email, user.email);
    assert_eq!(user_res.username, user.username);
}

#[sqlx::test]
async fn test_logout(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let response = env
        .server
        .post("/v1/auth/logout")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    // Note: This only invalidates the refresh token. The access token (JWT) is stateless
    // and will remain valid until it expires. A robust implementation would require a
    // token blacklist, but for now, we just verify the logout endpoint returns OK.
}

#[sqlx::test]
async fn test_login_wrong_password(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();

    let response = env
        .server
        .post("/v1/auth/login")
        .json(&serde_json::json!({
            "email": user.email,
            "password": "WrongPassword!"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_login_non_existent_user(pool: PgPool) {
    // No user is seeded in the database
    let env = TestEnvironmentBuilder::new(pool).build().await;

    let response = env
        .server
        .post("/v1/auth/login")
        .json(&serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "Password123!"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_me_no_token(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool).build().await;

    let response = env.server.get("/v1/auth/me").await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_me_invalid_token(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool).build().await;

    let response = env
        .server
        .get("/v1/auth/me")
        .add_header("Authorization", "Bearer invalidtoken")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_update_profile_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let new_username = "new-test-username";
    let new_email = "new-email@test.com";

    let update_response = env
        .server
        .put("/v1/auth/profile")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({
            "username": new_username,
            "email": new_email
        }))
        .await;

    update_response.assert_status_ok();

    // Verify the change was persisted by calling /me
    let me_response = env
        .server
        .get("/v1/auth/me")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    me_response.assert_status_ok();
    let user_json = me_response.json::<UserResponse>();

    assert_eq!(user_json.username, new_username);
    assert_eq!(user_json.email, new_email);
}

#[sqlx::test]
async fn test_change_password_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let new_password = "NewPassword456!";

    let response = env
        .server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({
            "current_password": TEST_PASSWORD,
            "new_password": new_password,
            "confirm_password": new_password
        }))
        .await;

    response.assert_status_ok();

    // Try to login with the new password
    let login_response = env
        .server
        .post("/v1/auth/login")
        .json(&serde_json::json!({
            "email": user.email,
            "password": new_password
        }))
        .await;

    login_response.assert_status_ok();
}

#[sqlx::test]
async fn test_change_password_wrong_current_password(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let new_password = "NewPassword456!";

    let response = env
        .server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({
            "current_password": "WrongPassword!",
            "new_password": new_password,
            "confirm_password": new_password
        }))
        .await;

    response.assert_status(StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn test_change_password_mismatched_new_password(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;
    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let response = env
        .server
        .put("/v1/auth/password")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({
            "current_password": TEST_PASSWORD,
            "new_password": "NewPassword456!",
            "confirm_password": "MismatchedPassword!"
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
}
