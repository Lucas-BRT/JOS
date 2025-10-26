use crate::utils::{TestEnvironmentBuilder, register_and_login};
use api::http::dtos::table::*;
use axum::http::StatusCode;
use domain::entities::{CreateTableMemberCommand, Table};
use sqlx::PgPool;

const GM_USER_ID: &str = "gm_user";
const PLAYER_USER_ID: &str = "player_user";
const GAME_SYSTEM_ID: &str = "default";
const TABLE_ID: &str = "test_table";
const TEST_PASSWORD: &str = "Password123!";

#[sqlx::test]
async fn test_create_table_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;
    let game_system = env.seeded.game_systems.get(GAME_SYSTEM_ID).unwrap();

    let req = CreateTableRequest {
        title: "My Awesome Table".to_string(),
        description: "A table for awesome people.".to_string(),
        system_id: game_system.id,
        max_players: 5,
    };

    let response = env
        .server
        .post("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .await;

    response.assert_status(StatusCode::CREATED);
}

#[sqlx::test]
async fn test_get_tables_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get("/v1/tables")
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    let tables_json = response.json::<Vec<Table>>();
    assert_eq!(tables_json.len(), 1);

    let table_item = &tables_json[0];
    assert_eq!(table_item.id, table.id);
    assert_eq!(table_item.title, table.title);
    assert_eq!(table_item.game_system_id, table.game_system_id);
}

#[sqlx::test]
async fn test_get_tables_with_search_succeeds(pool: PgPool) {
    let first_table_name = "The Dark Castle";
    let second_table_name = "A wrong gnoll";

    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_user(PLAYER_USER_ID)
        .with_table(first_table_name, GM_USER_ID)
        .with_table(second_table_name, PLAYER_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table1 = env.seeded.tables.get(first_table_name).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    println!("orangotango");
    let response = env
        .server
        .get(&format!("/v1/tables?gmId={}", gm.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    let tables_json = response.json::<Vec<Table>>();

    tables_json.iter().for_each(|t| println!("{t:?}"));
    assert_eq!(tables_json.len(), 1);
    assert_eq!(tables_json[0].id, table1.id);
}

#[sqlx::test]
async fn test_get_table_details_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.title, table.title);
    assert_eq!(table_json.id, table.id);
}

#[sqlx::test]
async fn test_get_table_details_game_system_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let game_system = env.seeded.game_systems.get("default").unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.game_system, game_system.name);
}

#[sqlx::test]
async fn test_get_table_details_players_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_user(PLAYER_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let player = env.seeded.users.get(PLAYER_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    // Add player to the table
    let create_table_member_command = CreateTableMemberCommand {
        table_id: table.id,
        user_id: player.id,
    };
    env.state
        .table_member_service
        .create(create_table_member_command)
        .await
        .unwrap();

    let response = env
        .server
        .get(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.players.len(), 1);
    assert_eq!(table_json.players[0].id, player.id);
    assert_eq!(table_json.players[0].username, player.username);
}

#[sqlx::test]
async fn test_update_table_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let req = UpdateTableRequest {
        title: Some("My Updated Table".to_string()),
        description: None,
        max_players: Some(10),
        system: None,
        visibility: None,
        status: None,
    };

    let response = env
        .server
        .put(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.title, req.title.unwrap());
    assert_eq!(table_json.player_slots, req.max_players.unwrap());
}

#[sqlx::test]
async fn test_update_table_game_system_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let new_game_system = env.seeded.game_systems.get("default").unwrap(); // Assuming "default" is a valid game system

    let req = UpdateTableRequest {
        title: None,
        description: None,
        max_players: None,
        system: Some(new_game_system.id.to_string()), // Update game system
        visibility: None,
        status: None,
    };

    let response = env
        .server
        .put(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.game_system, new_game_system.name);
}

#[sqlx::test]
async fn test_update_table_players_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_user(PLAYER_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let player = env.seeded.users.get(PLAYER_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    // Add player to the table
    let create_table_member_command = CreateTableMemberCommand {
        table_id: table.id,
        user_id: player.id,
    };
    env.state
        .table_member_service
        .create(create_table_member_command)
        .await
        .unwrap();

    let req = UpdateTableRequest {
        title: Some("Updated Title".to_string()),
        description: None,
        max_players: None,
        system: None,
        visibility: None,
        status: None,
    };

    let response = env
        .server
        .put(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.players.len(), 1);
    assert_eq!(table_json.players[0].id, player.id);
    assert_eq!(table_json.players[0].username, player.username);
}

#[sqlx::test]
async fn test_update_table_fails_for_non_gm(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_user(PLAYER_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let player = env.seeded.users.get(PLAYER_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &player.email, TEST_PASSWORD).await;

    let req = UpdateTableRequest {
        title: Some("Player Trying to Update".to_string()),
        description: None,
        max_players: None,
        system: None,
        visibility: None,
        status: None,
    };

    let response = env
        .server
        .put(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .json(&req)
        .await;

    response.assert_status(StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn test_delete_table_fails_for_non_gm(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_user(PLAYER_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let player = env.seeded.users.get(PLAYER_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &player.email, TEST_PASSWORD).await;

    let response = env
        .server
        .delete(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn test_delete_table_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let response = env
        .server
        .delete(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();

    // Verify it's gone
    let get_response = env
        .server
        .get(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    get_response.assert_status(StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn test_get_table_details_status_and_sessions_succeeds(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool)
        .with_user(GM_USER_ID)
        .with_table(TABLE_ID, GM_USER_ID)
        .with_session("test_session", TABLE_ID)
        .build()
        .await;

    let gm = env.seeded.users.get(GM_USER_ID).unwrap();
    let table = env.seeded.tables.get(TABLE_ID).unwrap();
    let session = env.seeded.sessions.get("test_session").unwrap();
    let token = register_and_login(&env.server, &gm.email, TEST_PASSWORD).await;

    let response = env
        .server
        .get(&format!("/v1/tables/{}", table.id))
        .add_header("Authorization", &format!("Bearer {}", token))
        .await;

    response.assert_status_ok();
    let table_json = response.json::<TableDetails>();
    assert_eq!(table_json.status, table.status.to_string());
    assert_eq!(table_json.sessions.len(), 1);
    assert_eq!(table_json.sessions[0].id, session.id);
    assert_eq!(table_json.sessions[0].title, session.name);
}
