#[path = "./utils/mod.rs"]
mod utils;

use api::http::dtos::UserResponse;
use axum::http::StatusCode;

use serde_json::json;
use uuid::Uuid;
use utils::api::{setup_test_environment, register_and_login};

#[tokio::test]
async fn test_update_profile_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

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
