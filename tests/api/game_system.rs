use std::fmt::format;

use crate::utils::{
    DEFAULT_USER_PASSWORD, TestEnvironment, TestEnvironmentBuilder, register_and_login,
};
use api::http::handlers::game_system::GameSystemResponse;
use axum::http::header::AUTHORIZATION;
use serde_json::json;
use sqlx::PgPool;
const TEST_USER_ID: &str = "camaraoasd";

async fn create_game_system(env: &TestEnvironment, name: &str, user_token: &str) {
    let response = env
        .server
        .post("/v1/game_systems")
        .json(&json!({
            "name": name
        }))
        .add_header("Authorization", &format!("Bearer {user_token}",))
        .await;

    response.assert_status_ok();
}

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

    create_game_system(&env, game_name, &token).await;
}

#[sqlx::test]
async fn test_get_all_game_systems(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let game_name = "D&D 5E";
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    assert!(!token.is_empty());

    create_game_system(&env, game_name, &token).await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    assert!(!token.is_empty());

    let response = env
        .server
        .get("/v1/game_systems")
        .add_header(AUTHORIZATION, format!("Bearer {}", token))
        .await;
    response.assert_status_ok();

    let response = response.json::<Vec<GameSystemResponse>>();

    assert!(response.iter().any(|gs| gs.name == game_name))
}

#[sqlx::test]
async fn test_get_all_game_systems_empty(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, DEFAULT_USER_PASSWORD).await;

    assert!(!token.is_empty());

    let response = env
        .server
        .get("/v1/game_systems")
        .add_header(AUTHORIZATION, format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    let response = response.json::<Vec<GameSystemResponse>>();

    println!("{response:#?}");

    assert!(response.is_empty())
}
