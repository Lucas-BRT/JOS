mod utils;

use jos::Error;
use jos::domain::session::filters::SessionFilters;
use jos::domain::session::{
    CreateSessionCommand, DeleteSessionCommand, GetSessionCommand, SessionRepository,
    UpdateSessionCommand,
};
use jos::domain::table::commands::CreateTableCommand;
use jos::domain::table::entity::Visibility;
use jos::domain::table::table_repository::TableRepository;
use jos::domain::utils::pagination::Pagination;
use jos::domain::utils::update::Update;
use jos::infrastructure::prelude::{PostgresSessionRepository, PostgresTableRepository};
use jos::infrastructure::repositories::error::RepositoryError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_session_success(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Test Session".to_string(),
        "A test session".to_string(),
        true,
    );

    let result = session_repo.create(&session_data).await;

    if let Ok(session) = result {
        assert_eq!(session.name, "Test Session");
        assert_eq!(session.description, "A test session".to_string());
        assert!(session.accepting_intents);
        assert_eq!(session.table_id, table.id);
        assert!(session.id != Uuid::nil());
    }
}

#[sqlx::test]
async fn test_create_session_accepting_intents_false(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Closed Session".to_string(),
        "A closed test session".to_string(),
        false,
    );

    let result = session_repo.create(&session_data).await;

    if let Ok(session) = result {
        assert_eq!(session.name, "Closed Session");
        assert!(!session.accepting_intents);
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Test Session".to_string(),
        "A test session".to_string(),
        true,
    );

    let created_session = session_repo.create(&session_data).await.unwrap();
    let found_session = session_repo.find_by_id(&created_session.id).await.unwrap();

    if let Some(session) = found_session {
        assert_eq!(session.id, created_session.id);
        assert_eq!(session.name, "Test Session");
        assert_eq!(session.description, "A test session".to_string());
        assert!(session.accepting_intents);
    } else {
        panic!("Session not found");
    }
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));

    let random_id = Uuid::new_v4();
    let result = session_repo.find_by_id(&random_id).await.unwrap();

    if let Some(session) = result {
        panic!("Unexpected session found: {session:?}");
    }
}

#[sqlx::test]
async fn test_find_by_table_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table for RPG".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data1 = CreateSessionCommand::new(
        table.id,
        "Session 1".to_string(),
        "First session".to_string(),
        true,
    );

    let session_data2 = CreateSessionCommand::new(
        table.id,
        "Session 2".to_string(),
        "Second session".to_string(),
        false,
    );

    session_repo.create(&session_data1).await.unwrap();
    session_repo.create(&session_data2).await.unwrap();

    let found_sessions = session_repo.find_by_table_id(&table.id).await.unwrap();
    assert_eq!(found_sessions.len(), 2);

    let names: Vec<String> = found_sessions.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"Session 1".to_string()));
    assert!(names.contains(&"Session 2".to_string()));
}

#[sqlx::test]
async fn test_find_by_table_id_empty(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(Arc::new(pool));
    let random_table_id = Uuid::new_v4();

    let found_sessions = session_repo
        .find_by_table_id(&random_table_id)
        .await
        .unwrap();
    assert_eq!(found_sessions.len(), 0);
}

#[sqlx::test]
async fn test_get_all_sessions(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm1 = utils::create_user(&pool).await;
    let gm2 = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data1 = CreateTableCommand::new(
        gm1.id,
        "Table 1".into(),
        "First table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );

    let table_data2 = CreateTableCommand::new(
        gm2.id,
        "Table 2".into(),
        "Second table".into(),
        Visibility::Public,
        3,
        game_system.id,
    );

    let table1 = table_repo.create(&table_data1).await.unwrap();
    let table2 = table_repo.create(&table_data2).await.unwrap();

    let session_data1 = CreateSessionCommand::new(
        table1.id,
        "Session 1".to_string(),
        "First session".to_string(),
        true,
    );

    let session_data2 = CreateSessionCommand::new(
        table2.id,
        "Session 2".to_string(),
        "Second session".to_string(),
        false,
    );

    session_repo.create(&session_data1).await.unwrap();
    session_repo.create(&session_data2).await.unwrap();

    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo.get(&get_command).await.unwrap();
    assert_eq!(all_sessions.len(), 2);

    let names: Vec<String> = all_sessions.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"Session 1".to_string()));
    assert!(names.contains(&"Session 2".to_string()));
}

#[sqlx::test]
async fn test_get_sessions_with_filters(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data1 = CreateSessionCommand {
        table_id: table.id,
        name: "Open Session".to_string(),
        description: "Open session".to_string(),
        accepting_intents: true,
    };

    let session_data2 = CreateSessionCommand {
        table_id: table.id,
        name: "Closed Session".to_string(),
        description: "Closed session".to_string(),
        accepting_intents: false,
    };

    session_repo.create(&session_data1).await.unwrap();
    session_repo.create(&session_data2).await.unwrap();

    let filters = SessionFilters::default().with_accepting_intents(true);
    let get_command = GetSessionCommand::default().with_filters(filters);
    let open_sessions = session_repo.get(&get_command).await.unwrap();

    assert_eq!(open_sessions.len(), 1);
    assert_eq!(open_sessions[0].name, "Open Session");
    assert!(open_sessions[0].accepting_intents);
}

#[sqlx::test]
async fn test_update_session_name(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Original Name".to_string(),
        "A test session".to_string(),
        true,
    );

    let created_session = session_repo.create(&session_data).await.unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        name: Update::Change("Updated Name".to_string()),
        description: Update::Keep,
        accepting_intents: Update::Keep,
    };

    session_repo.update(&update_data).await.unwrap();

    let updated_session = session_repo.find_by_id(&created_session.id).await.unwrap();

    match updated_session {
        Some(session) => {
            assert_eq!(session.name, "Updated Name");
            assert_eq!(session.description, "A test session".to_string());
            assert!(session.accepting_intents);
        }
        None => panic!("Session not found"),
    }
}

#[sqlx::test]
async fn test_update_session_description(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Test Session".to_string(),
        "Original description".to_string(),
        true,
    );

    let created_session = session_repo.create(&session_data).await.unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        name: Update::Keep,
        description: Update::Change("Updated description".to_string()),
        accepting_intents: Update::Keep,
    };

    session_repo.update(&update_data).await.unwrap();

    let updated_session = session_repo.find_by_id(&created_session.id).await.unwrap();

    match updated_session {
        Some(session) => {
            assert_eq!(session.name, "Test Session");
            assert_eq!(session.description, "Updated description".to_string());
        }
        None => panic!("Session not found"),
    }
}

#[sqlx::test]
async fn test_update_session_accepting_intents(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        accepting_intents: true,
    };

    let created_session = session_repo.create(&session_data).await.unwrap();

    let update_data = UpdateSessionCommand::new(
        created_session.id,
        Update::Keep,
        Update::Keep,
        Update::Change(false),
    );

    session_repo.update(&update_data).await.unwrap();

    let updated_session = session_repo.find_by_id(&created_session.id).await.unwrap();

    match updated_session {
        Some(session) => {
            assert!(!session.accepting_intents);
        }
        None => {
            panic!("Session not found");
        }
    }
}

#[sqlx::test]
async fn test_update_session_multiple_fields(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Original Name".to_string(),
        "Original description".to_string(),
        true,
    );

    let created_session = session_repo.create(&session_data).await.unwrap();

    let update_data = UpdateSessionCommand::new(
        created_session.id,
        Update::Change("New Name".to_string()),
        Update::Change("New description".to_string()),
        Update::Change(false),
    );

    session_repo.update(&update_data).await.unwrap();

    let updated_session = session_repo.find_by_id(&created_session.id).await.unwrap();

    match updated_session {
        Some(session) => {
            assert_eq!(session.name, "New Name");
            assert_eq!(session.description, "New description".to_string());
            assert!(!session.accepting_intents);
        }
        None => panic!("Session not found"),
    }
}

#[sqlx::test]
async fn test_update_session_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(Arc::new(pool));

    let random_id = Uuid::new_v4();
    let update_data = UpdateSessionCommand {
        id: random_id,
        name: Update::Change("New Name".to_string()),
        description: Update::Keep,
        accepting_intents: Update::Keep,
    };

    let result = session_repo.update(&update_data).await;

    match result {
        Err(Error::Repository(RepositoryError::NotFound)) => (),
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_delete_session(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let session_data = CreateSessionCommand::new(
        table.id,
        "Test Session".to_string(),
        "A test session".to_string(),
        true,
    );

    let created_session = session_repo
        .create(&session_data)
        .await
        .expect("Failed to create session");

    let delete_command = DeleteSessionCommand::new(created_session.id);

    let deleted_session = session_repo
        .delete(&delete_command)
        .await
        .expect("Failed to delete session");

    assert_eq!(deleted_session.id, created_session.id);
    assert_eq!(deleted_session.name, "Test Session");
}

#[sqlx::test]
async fn test_delete_session_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(Arc::new(pool));

    let random_id = Uuid::new_v4();
    let delete_command = DeleteSessionCommand::new(random_id);

    let result = session_repo.delete(&delete_command).await;

    match result {
        Err(Error::Repository(RepositoryError::NotFound)) => (),
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_session_operations(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let pool = pool.clone();
            let session_data = CreateSessionCommand::new(
                table.id,
                format!("Session {i}"),
                format!("Description for session {i}"),
                i % 2 == 0,
            );
            tokio::spawn(async move {
                let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
                session_repo
                    .create(&session_data)
                    .await
                    .expect("Failed to create session")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo
        .get(&get_command)
        .await
        .expect("Failed to get all sessions");
    assert_eq!(all_sessions.len(), 5);
}

#[sqlx::test]
async fn test_pagination(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let table_data = CreateTableCommand::new(
        gm.id,
        "Test Table".into(),
        "A test table".into(),
        Visibility::Public,
        5,
        game_system.id,
    );
    let table = table_repo.create(&table_data).await.unwrap();

    // Create 25 sessions
    for i in 0..25 {
        let session_data = CreateSessionCommand::new(
            table.id,
            format!("Session {i}"),
            format!("Description {i}"),
            true,
        );

        session_repo
            .create(&session_data)
            .await
            .expect("Failed to create session");
    }

    // Test first page (assuming default page size is 20)
    let pagination = Pagination::default();
    let get_command = GetSessionCommand::default().with_pagination(pagination);
    let first_page = session_repo
        .get(&get_command)
        .await
        .expect("Failed to get first page");
    assert_eq!(first_page.len(), 20);

    // Test second page
    let pagination = Pagination::default().with_page(2);
    let get_command = GetSessionCommand::default().with_pagination(pagination);
    let second_page = session_repo
        .get(&get_command)
        .await
        .expect("Failed to get second page");
    assert_eq!(second_page.len(), 5);

    // Test custom page size
    let pagination = Pagination::default().with_page_size(10);
    let get_command = GetSessionCommand::default().with_pagination(pagination);
    let custom_page = session_repo
        .get(&get_command)
        .await
        .expect("Failed to get custom page");
    assert_eq!(custom_page.len(), 10);
}
