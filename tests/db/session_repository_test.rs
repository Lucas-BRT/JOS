#[path = "./utils/mod.rs"]
mod utils;

use jos::domain::entities::commands::{
    CreateSessionCommand, CreateTableCommand, DeleteSessionCommand, GetSessionCommand,
    UpdateSessionCommand,
};
use jos::domain::entities::update::Update;
use jos::domain::repositories::{SessionRepository, TableRepository};
use jos::infrastructure::persistence::postgres::repositories::{
    PostgresSessionRepository, PostgresTableRepository,
};
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_session_success(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let result = session_repo.create(session_data).await;

    match result {
        Ok(session) => {
            assert_eq!(session.name, "Test Session");
            assert_eq!(session.description, "A test session");
            assert_eq!(session.table_id, table.id);
            assert_eq!(
                session.status,
                jos::domain::entities::SessionStatus::Scheduled
            );
            assert!(session.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let created_session = session_repo.create(session_data).await.unwrap();
    let get_command = GetSessionCommand {
        id: Some(created_session.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();

    assert_eq!(found_sessions.len(), 1);
    let found_session = &found_sessions[0];
    assert_eq!(found_session.id, created_session.id);
    assert_eq!(found_session.name, "Test Session");
    assert_eq!(found_session.description, "A test session");
    assert_eq!(found_session.table_id, table.id);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(pool);

    let random_id = Uuid::new_v4();
    let get_command = GetSessionCommand {
        id: Some(random_id),
        ..Default::default()
    };
    let result = session_repo.read(get_command).await;

    assert!(result.is_ok());
    let found_sessions = result.unwrap();
    assert!(found_sessions.is_empty());
}

#[sqlx::test]
async fn test_find_by_table_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data1 = CreateSessionCommand {
        table_id: table.id,
        name: "Session 1".to_string(),
        description: "First session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };
    let session_data2 = CreateSessionCommand {
        table_id: table.id,
        name: "Session 2".to_string(),
        description: "Second session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Completed,
    };

    session_repo.create(session_data1).await.unwrap();
    session_repo.create(session_data2).await.unwrap();

    let get_command = GetSessionCommand {
        table_id: Some(table.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(found_sessions.len(), 2);

    let names: Vec<String> = found_sessions.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"Session 1".to_string()));
    assert!(names.contains(&"Session 2".to_string()));
}

#[sqlx::test]
async fn test_get_all_sessions(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm1 = utils::create_user(&pool).await;
    let gm2 = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand {
        gm_id: gm1.id,
        title: "Table 1".to_string(),
        description: "First table".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table1 = table_repo.create(table_data1).await.unwrap();

    let table_data2 = CreateTableCommand {
        gm_id: gm2.id,
        title: "Table 2".to_string(),
        description: "Second table".to_string(),
        slots: 3,
        game_system_id: game_system.id,
    };
    let table2 = table_repo.create(table_data2).await.unwrap();

    let session_data1 = CreateSessionCommand {
        table_id: table1.id,
        name: "Session 1".to_string(),
        description: "First session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };
    let session_data2 = CreateSessionCommand {
        table_id: table2.id,
        name: "Session 2".to_string(),
        description: "Second session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Completed,
    };

    session_repo.create(session_data1).await.unwrap();
    session_repo.create(session_data2).await.unwrap();

    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(all_sessions.len(), 2);

    let names: Vec<String> = all_sessions.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"Session 1".to_string()));
    assert!(names.contains(&"Session 2".to_string()));
}

#[sqlx::test]
async fn test_update_session_name(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Original Name".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let created_session = session_repo.create(session_data).await.unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        name: Update::Change("Updated Name".to_string()),
        ..Default::default()
    };

    let result = session_repo.update(update_data).await;
    assert!(result.is_ok());

    let get_command = GetSessionCommand {
        id: Some(created_session.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(found_sessions.len(), 1);
    let updated_session = &found_sessions[0];
    assert_eq!(updated_session.name, "Updated Name");
    assert_eq!(updated_session.description, "A test session");
}

#[sqlx::test]
async fn test_update_session_description(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "Original Description".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let created_session = session_repo.create(session_data).await.unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        description: Update::Change("Updated Description".to_string()),
        ..Default::default()
    };

    session_repo
        .update(update_data)
        .await
        .expect("Failed to update session");

    let get_command = GetSessionCommand {
        id: Some(created_session.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(found_sessions.len(), 1);
    let updated_session = &found_sessions[0];
    assert_eq!(updated_session.description, "Updated Description");
}

#[sqlx::test]
async fn test_update_session_status(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let created_session = session_repo.create(session_data).await.unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        status: Update::Change(jos::domain::entities::SessionStatus::Completed),
        ..Default::default()
    };

    session_repo
        .update(update_data)
        .await
        .expect("Failed to update session");

    let get_command = GetSessionCommand {
        id: Some(created_session.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(found_sessions.len(), 1);
    let updated_session = &found_sessions[0];
    assert_eq!(
        updated_session.status,
        jos::domain::entities::SessionStatus::Completed
    );
}

#[sqlx::test]
async fn test_delete_session(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 5,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };

    let created_session = session_repo
        .create(session_data)
        .await
        .expect("Failed to create session");

    let delete_command = DeleteSessionCommand {
        id: created_session.id,
    };

    let deleted_session = session_repo
        .delete(delete_command)
        .await
        .expect("Failed to delete session");

    assert_eq!(deleted_session.id, created_session.id);
    assert_eq!(deleted_session.name, "Test Session");
}

#[sqlx::test]
async fn test_delete_session_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(pool);

    let random_id = Uuid::new_v4();
    let delete_command = DeleteSessionCommand { id: random_id };
    let result = session_repo.delete(delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_session_operations(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand {
        gm_id: gm.id,
        title: "Test Table".to_string(),
        description: "A test table for RPG".to_string(),
        slots: 10,
        game_system_id: game_system.id,
    };
    let table = table_repo.create(table_data).await.unwrap();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let pool = pool.clone();
            let session_data = CreateSessionCommand {
                table_id: table.id,
                name: format!("Session {i}"),
                description: format!("Description for session {i}"),
                scheduled_for: None,
                status: jos::domain::entities::SessionStatus::Scheduled,
            };
            tokio::spawn(async move {
                let session_repo = PostgresSessionRepository::new(pool.clone());
                session_repo
                    .create(session_data)
                    .await
                    .expect("Failed to create session")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo
        .read(get_command)
        .await
        .expect("Failed to get all sessions");
    assert_eq!(all_sessions.len(), 5);
}
