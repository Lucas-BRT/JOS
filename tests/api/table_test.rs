use super::utils::api::{register_and_login, setup_test_environment};
use api::http::dtos::TableDetails;
use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn test_create_table_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    let response = server
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

    response.assert_status(StatusCode::CREATED);
    let table_json = response.json::<TableDetails>();

    assert_eq!(table_json.title, title);
    assert_eq!(table_json.game_system, system);
    assert_eq!(table_json.description, description);
    assert_eq!(table_json.player_slots, max_players);
}

#[tokio::test]
async fn test_get_tables_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
    let token = register_and_login(&server).await;

    let title = "My Awesome Table";
    let system = "D&D 5e";
    let description = "A table for awesome people.";
    let max_players = 5;

    server
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

    let response = server
        .get("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);
    let tables_json = response.json::<Vec<TableDetails>>();
    assert!(!tables_json.is_empty());
}

#[tokio::test]
async fn test_get_table_details_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
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

#[tokio::test]
async fn test_update_table_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
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

#[tokio::test]
async fn test_delete_table_succeeds() {
    let (server, _pool, _mock_server) = setup_test_environment().await;
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
