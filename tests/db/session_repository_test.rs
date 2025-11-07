use crate::utils::TestEnvironmentBuilder;
use domain::entities::SessionStatus;
use jos::domain::entities::commands::*;
use jos::domain::entities::update::Update;
use jos::domain::repositories::*;
use jos::infrastructure::persistence::postgres::repositories::*;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

const GM_ID: &str = "gm";
const TABLE_ID: &str = "table1";

#[sqlx::test]
async fn test_create_session_success(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        title: "Test Session".to_string(),
        description: "A session for testing".to_string(),
        scheduled_for: None,
        status: SessionStatus::Scheduled,
    };

    let result = session_repo.create(session_data).await;

    match result {
        Ok(session) => {
            assert_eq!(session.table_id, table.id);
            assert_eq!(session.title, "Test Session");
            assert!(session.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let session_data = CreateSessionCommand {
        table_id: table.id,
        title: "Test Session".to_string(),
        description: "A session for testing".to_string(),
        scheduled_for: None,
        status: SessionStatus::Scheduled,
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
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let session_repo = PostgresSessionRepository::new(pool);
    let random_id = Uuid::new_v4();
    let get_command = GetSessionCommand {
        id: Some(random_id),
        ..Default::default()
    };
    let result = session_repo.read(get_command).await.unwrap();
    assert!(result.is_empty());
}

#[sqlx::test]
async fn test_find_by_table_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_table("table2", GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table1 = env.seeded.tables.get(TABLE_ID).unwrap();
    let table2 = env.seeded.tables.get("table2").unwrap();

    session_repo
        .create(CreateSessionCommand {
            table_id: table1.id,
            title: "Session 1".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();
    session_repo
        .create(CreateSessionCommand {
            table_id: table2.id,
            title: "Session 2".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let get_command = GetSessionCommand {
        table_id: Some(table1.id),
        ..Default::default()
    };
    let found_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(found_sessions.len(), 1);
    assert_eq!(found_sessions[0].title, "Session 1");
}

#[sqlx::test]
async fn test_get_all_sessions(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_table("table2", GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table1 = env.seeded.tables.get(TABLE_ID).unwrap();
    let table2 = env.seeded.tables.get("table2").unwrap();

    session_repo
        .create(CreateSessionCommand {
            table_id: table1.id,
            title: "Session 1".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();
    session_repo
        .create(CreateSessionCommand {
            table_id: table2.id,
            title: "Session 2".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo.read(get_command).await.unwrap();
    assert_eq!(all_sessions.len(), 2);
}

#[sqlx::test]
async fn test_update_session_title(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_session = session_repo
        .create(CreateSessionCommand {
            table_id: table.id,
            title: "Original Name".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        title: Update::Change("New Name".to_string()),
        ..Default::default()
    };

    session_repo.update(update_data).await.unwrap();

    let found_sessions = session_repo
        .read(GetSessionCommand {
            id: Some(created_session.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(found_sessions[0].title, "New Name");
}

#[sqlx::test]
async fn test_update_session_description(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_session = session_repo
        .create(CreateSessionCommand {
            table_id: table.id,
            title: "Test Session".to_string(),
            description: "Original Description".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        description: Update::Change("New Description".to_string()),
        ..Default::default()
    };

    session_repo.update(update_data).await.unwrap();

    let found_sessions = session_repo
        .read(GetSessionCommand {
            id: Some(created_session.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(found_sessions[0].description, "New Description");
}

#[sqlx::test]
async fn test_update_session_status(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_session = session_repo
        .create(CreateSessionCommand {
            table_id: table.id,
            title: "Test Session".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let update_data = UpdateSessionCommand {
        id: created_session.id,
        status: Update::Change(SessionStatus::Completed),
        ..Default::default()
    };

    session_repo.update(update_data).await.unwrap();

    let found_sessions = session_repo
        .read(GetSessionCommand {
            id: Some(created_session.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(found_sessions[0].status, SessionStatus::Completed);
}

#[sqlx::test]
async fn test_delete_session(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let created_session = session_repo
        .create(CreateSessionCommand {
            table_id: table.id,
            title: "To Be Deleted".to_string(),
            description: "".to_string(),
            scheduled_for: None,
            status: SessionStatus::Scheduled,
        })
        .await
        .unwrap();

    let delete_command = DeleteSessionCommand {
        id: created_session.id,
    };

    let deleted_session = session_repo.delete(delete_command).await.unwrap();
    assert_eq!(deleted_session.id, created_session.id);

    let result = session_repo
        .read(GetSessionCommand {
            id: Some(created_session.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(result.is_empty());
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
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_table(TABLE_ID, GM_ID)
        .build()
        .await;
    let table = env.seeded.tables.get(TABLE_ID).unwrap();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let pool = pool.clone();
            let table_id = table.id;
            tokio::spawn(async move {
                let session_data = CreateSessionCommand {
                    table_id,
                    title: format!("Session {}", i),
                    description: format!("Description for session {}", i),
                    scheduled_for: None,
                    status: SessionStatus::Scheduled,
                };
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

    let session_repo = PostgresSessionRepository::new(pool.clone());
    let get_command = GetSessionCommand::default();
    let all_sessions = session_repo
        .read(get_command)
        .await
        .expect("Failed to get all sessions");
    assert_eq!(all_sessions.len(), 5);
}
