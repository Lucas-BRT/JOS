#[path = "../utils/mod.rs"]
mod utils;

use jos::domain::entities::commands::{
    CreateSessionCommand, CreateSessionIntentCommand, CreateTableCommand,
    DeleteSessionIntentCommand, GetSessionIntentCommand, UpdateSessionIntentCommand,
};
use jos::domain::entities::session_intent::IntentStatus;
use jos::domain::entities::update::Update;
use jos::domain::repositories::{SessionIntentRepository, SessionRepository, TableRepository};
use jos::infrastructure::persistence::postgres::repositories::{
    PostgresSessionIntentRepository, PostgresSessionRepository, PostgresTableRepository,
};
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_session_intent_success(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
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
    let session = session_repo.create(session_data).await.unwrap();

    let intent_data = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session.id,
        status: IntentStatus::Confirmed,
    };

    let result = session_intent_repo.create(intent_data).await;

    match result {
        Ok(intent) => {
            assert_eq!(intent.user_id, user.id);
            assert_eq!(intent.session_id, session.id);
            assert_eq!(intent.intent_status, IntentStatus::Confirmed);
            assert!(intent.id != Uuid::nil());
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
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
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
    let session = session_repo.create(session_data).await.unwrap();

    let intent_data = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session.id,
        status: IntentStatus::Confirmed,
    };

    let created_intent = session_intent_repo.create(intent_data).await.unwrap();
    let get_command = GetSessionIntentCommand {
        id: Some(created_intent.id),
        ..Default::default()
    };
    let found_intents = session_intent_repo.read(get_command).await.unwrap();

    assert_eq!(found_intents.len(), 1);
    let found_intent = &found_intents[0];
    assert_eq!(found_intent.id, created_intent.id);
    assert_eq!(found_intent.user_id, user.id);
    assert_eq!(found_intent.session_id, session.id);
    assert_eq!(found_intent.intent_status, IntentStatus::Confirmed);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(pool);

    let random_id = Uuid::new_v4();
    let get_command = GetSessionIntentCommand {
        id: Some(random_id),
        ..Default::default()
    };
    let result = session_intent_repo.read(get_command).await;

    assert!(result.is_ok());
    let found_intents = result.unwrap();
    assert!(found_intents.is_empty());
}

#[sqlx::test]
async fn test_find_by_user_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
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
    let session1 = session_repo.create(session_data1).await.unwrap();

    let session_data2 = CreateSessionCommand {
        table_id: table.id,
        name: "Session 2".to_string(),
        description: "Second session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };
    let session2 = session_repo.create(session_data2).await.unwrap();

    let intent_data1 = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session1.id,
        status: IntentStatus::Confirmed,
    };
    let intent_data2 = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session2.id,
        status: IntentStatus::Tentative,
    };

    session_intent_repo.create(intent_data1).await.unwrap();
    session_intent_repo.create(intent_data2).await.unwrap();

    let get_command = GetSessionIntentCommand {
        user_id: Some(user.id),
        ..Default::default()
    };
    let found_intents = session_intent_repo.read(get_command).await.unwrap();
    assert_eq!(found_intents.len(), 2);

    let statuses: Vec<IntentStatus> = found_intents.iter().map(|i| i.intent_status).collect();
    assert!(statuses.contains(&IntentStatus::Confirmed));
    assert!(statuses.contains(&IntentStatus::Tentative));
}

#[sqlx::test]
async fn test_find_by_session_id(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user1 = utils::create_user(&pool).await;
    let user2 = utils::create_user(&pool).await;
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
    let session = session_repo.create(session_data).await.unwrap();

    let intent_data1 = CreateSessionIntentCommand {
        player_id: user1.id,
        session_id: session.id,
        status: IntentStatus::Confirmed,
    };
    let intent_data2 = CreateSessionIntentCommand {
        player_id: user2.id,
        session_id: session.id,
        status: IntentStatus::Tentative,
    };

    session_intent_repo.create(intent_data1).await.unwrap();
    session_intent_repo.create(intent_data2).await.unwrap();

    let get_command = GetSessionIntentCommand {
        session_id: Some(session.id),
        ..Default::default()
    };
    let found_intents = session_intent_repo.read(get_command).await.unwrap();
    assert_eq!(found_intents.len(), 2);

    let player_ids: Vec<Uuid> = found_intents.iter().map(|i| i.user_id).collect();
    assert!(player_ids.contains(&user1.id));
    assert!(player_ids.contains(&user2.id));
}

#[sqlx::test]
async fn test_get_all_session_intents(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user1 = utils::create_user(&pool).await;
    let user2 = utils::create_user(&pool).await;
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
    let session1 = session_repo.create(session_data1).await.unwrap();

    let session_data2 = CreateSessionCommand {
        table_id: table.id,
        name: "Session 2".to_string(),
        description: "Second session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };
    let session2 = session_repo.create(session_data2).await.unwrap();

    let intent_data1 = CreateSessionIntentCommand {
        player_id: user1.id,
        session_id: session1.id,
        status: IntentStatus::Confirmed,
    };
    let intent_data2 = CreateSessionIntentCommand {
        player_id: user2.id,
        session_id: session2.id,
        status: IntentStatus::Tentative,
    };

    session_intent_repo.create(intent_data1).await.unwrap();
    session_intent_repo.create(intent_data2).await.unwrap();

    let get_command = GetSessionIntentCommand::default();
    let all_intents = session_intent_repo.read(get_command).await.unwrap();
    assert_eq!(all_intents.len(), 2);

    let statuses: Vec<IntentStatus> = all_intents.iter().map(|i| i.intent_status).collect();
    assert!(statuses.contains(&IntentStatus::Confirmed));
    assert!(statuses.contains(&IntentStatus::Tentative));
}

#[sqlx::test]
async fn test_update_session_intent_status(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
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
    let session = session_repo.create(session_data).await.unwrap();

    let intent_data = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session.id,
        status: IntentStatus::Tentative,
    };

    let created_intent = session_intent_repo.create(intent_data).await.unwrap();

    let update_data = UpdateSessionIntentCommand {
        id: created_intent.id,
        status: Update::Change(IntentStatus::Confirmed),
    };

    let result = session_intent_repo.update(update_data).await;
    assert!(result.is_ok());

    let get_command = GetSessionIntentCommand {
        id: Some(created_intent.id),
        ..Default::default()
    };
    let found_intents = session_intent_repo.read(get_command).await.unwrap();
    assert_eq!(found_intents.len(), 1);
    let updated_intent = &found_intents[0];
    assert_eq!(updated_intent.intent_status, IntentStatus::Confirmed);
}

#[sqlx::test]
async fn test_delete_session_intent(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

    let user = utils::create_user(&pool).await;
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
    let session = session_repo.create(session_data).await.unwrap();

    let intent_data = CreateSessionIntentCommand {
        player_id: user.id,
        session_id: session.id,
        status: IntentStatus::Confirmed,
    };

    let created_intent = session_intent_repo
        .create(intent_data)
        .await
        .expect("Failed to create session intent");

    let delete_command = DeleteSessionIntentCommand {
        id: created_intent.id,
    };

    let deleted_intent = session_intent_repo
        .delete(delete_command)
        .await
        .expect("Failed to delete session intent");

    assert_eq!(deleted_intent.id, created_intent.id);
    assert_eq!(deleted_intent.user_id, user.id);
    assert_eq!(deleted_intent.session_id, session.id);
}

#[sqlx::test]
async fn test_delete_session_intent_not_found(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(pool);

    let random_id = Uuid::new_v4();
    let delete_command = DeleteSessionIntentCommand { id: random_id };
    let result = session_intent_repo.delete(delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_session_intent_operations(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(pool.clone());
    let session_repo = PostgresSessionRepository::new(pool.clone());
    let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());

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

    let session_data = CreateSessionCommand {
        table_id: table.id,
        name: "Test Session".to_string(),
        description: "A test session".to_string(),
        scheduled_for: None,
        status: jos::domain::entities::SessionStatus::Scheduled,
    };
    let session = session_repo.create(session_data).await.unwrap();

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                let user = utils::create_user(&pool).await;
                let intent_data = CreateSessionIntentCommand {
                    player_id: user.id,
                    session_id: session.id,
                    status: IntentStatus::Tentative,
                };
                let session_intent_repo = PostgresSessionIntentRepository::new(pool.clone());
                session_intent_repo
                    .create(intent_data)
                    .await
                    .expect("Failed to create session intent")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let get_command = GetSessionIntentCommand::default();
    let all_intents = session_intent_repo
        .read(get_command)
        .await
        .expect("Failed to get all session intents");
    assert_eq!(all_intents.len(), 5);
}
