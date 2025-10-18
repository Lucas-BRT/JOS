use crate::utils::{TestEnvironmentBuilder, register_and_login};
use axum::http::StatusCode;
use sqlx::PgPool;
use api::http::dtos::UserResponse;

const TEST_USER_ID: &str = "test_user";
const TEST_PASSWORD: &str = "Password123!";

#[sqlx::test]
async fn test_get_user_by_id_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get(&format!("/v1/user/{}", user.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let user_res = response.json::<UserResponse>();
    assert_eq!(user_res.id, user.id);
    assert_eq!(user_res.email, user.email);
    assert_eq!(user_res.username, user.username);
}

#[sqlx::test]
async fn test_get_user_by_id_not_found(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(TEST_USER_ID)
        .build()
        .await;

    let user = env.seeded.users.get(TEST_USER_ID).unwrap();
    let token = register_and_login(&env.server, &user.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get("/v1/user/non_existent_id")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn test_get_user_by_id_unauthorized(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool).build().await;

    let response = env
        .server
        .get(&format!("/v1/user/{}", "some_id"))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}
