use crate::utils::{DEFAULT_USER_PASSWORD, TestEnvironmentBuilder, register_and_login};
use serde_json::json;
use sqlx::PgPool;
const TEST_USER_ID: &str = "camaraoasd";

#[sqlx::test]
async fn test_create_game_system(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let game_name = "D&D";
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    assert!(!token.is_empty());
    let response = env
        .server
        .post("/v1/game_systems")
        .json(&json!({
            "name": game_name
        }))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
}
