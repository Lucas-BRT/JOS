use super::utils::{register_and_login, setup_test_environment};
use crate::db::utils::create_game_system;
use api::http::dtos::TableDetails;
use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_table_succeeds(pool: PgPool) {
    let (server, _mock_server) = setup_test_environment(&pool).await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let game_system = create_game_system(&pool).await;
    let description = "A table for awesome people.";
    let max_players = 5;

    let response = server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": title,
            "system_id": game_system.id,
            "description": description,
            "max_players": max_players,
        }))
        .await;

    response.assert_status(StatusCode::CREATED);
}

#[sqlx::test]
async fn test_get_tables_succeeds(pool: PgPool) {
    let (server, _mock_server) = setup_test_environment(&pool).await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    let create_response = server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": title,
            "system": system,
            "description": description,
            "max_players": max_players,
            "visibility": "public",
        }))
        .await;
    create_response.assert_status(StatusCode::CREATED);

    let response = server
        .get("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);
    let tables_json = response.json::<Vec<TableDetails>>();
    assert!(!tables_json.is_empty());
}

#[sqlx::test]
async fn test_get_table_details_succeeds(pool: PgPool) {
    let (server, _mock_server) = setup_test_environment(&pool).await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    let create_response = server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": title,
            "system": system,
            "description": description,
            "max_players": max_players,
            "visibility": "public",
        }))
        .await;

    let created_table = create_response.json::<TableDetails>();

    let response = server
        .get(&format!("/v1/tables/{}", created_table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.title, title);
}

#[sqlx::test]
async fn test_update_table_succeeds(pool: PgPool) {
    let (server, _mock_server) = setup_test_environment(&pool).await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    let create_response = server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": title,
            "system": system,
            "description": description,
            "max_players": max_players,
            "visibility": "public",
        }))
        .await;

    let created_table = create_response.json::<TableDetails>();

    let new_title = "My Updated Table";
    let response = server
        .put(&format!("/v1/tables/{}", created_table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": new_title,
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.title, new_title);
}

#[sqlx::test]
async fn test_delete_table_succeeds(pool: PgPool) {
    let (server, _mock_server) = setup_test_environment(&pool).await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    let create_response = server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&json!({
            "title": title,
            "system": system,
            "description": description,
            "max_players": max_players,
            "visibility": "public",
        }))
        .await;

    let created_table = create_response.json::<TableDetails>();

    let response = server
        .delete(&format!("/v1/tables/{}", created_table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);

    let response = server
        .get(&format!("/v1/tables/{}", created_table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::NOT_FOUND);
}
