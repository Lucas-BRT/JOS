use crate::utils::TestEnvironmentBuilder;
use jos::domain::entities::commands::*;
use jos::domain::entities::session_intent::IntentStatus;
use jos::domain::entities::update::Update;
use jos::domain::repositories::*;
use jos::infrastructure::persistence::postgres::repositories::*;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

const GM_ID: &str = "gm";
const PLAYER_ID: &str = "player";
const OTHER_PLAYER_ID: &str = "other_player";
const TABLE_ID: &str = "table1";
const SESSION_ID: &str = "session1";

#[sqlx::test]
async fn test_create_session_intent_success(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());

    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();

    let intent_data = CreateSessionIntentCommand {
        player_id: player.id,
        session_id: session.id,
        status: IntentStatus::Confirmed,
    };

    let result = repo.create(intent_data).await;

    match result {
        Ok(intent) => {
            assert_eq!(intent.user_id, player.id);
            assert_eq!(intent.session_id, session.id);
            assert_eq!(intent.intent_status, IntentStatus::Confirmed);
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
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();

    let created_intent = repo
        .create(CreateSessionIntentCommand {
            player_id: player.id,
            session_id: session.id,
            status: IntentStatus::Confirmed,
        })
        .await
        .unwrap();

    let found_intents = repo
        .read(GetSessionIntentCommand {
            id: Some(created_intent.id),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(found_intents.len(), 1);
    assert_eq!(found_intents[0].id, created_intent.id);
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let get_command = GetSessionIntentCommand {
        id: Some(Uuid::new_v4()),
        ..Default::default()
    };
    let found_intents = repo.read(get_command).await.unwrap();
    assert!(found_intents.is_empty());
}

#[sqlx::test]
async fn test_find_by_user_id(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .with_session("session2", TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let session1 = env.seeded.sessions.get(SESSION_ID).unwrap();
    let session2 = env.seeded.sessions.get("session2").unwrap();

    repo.create(CreateSessionIntentCommand {
        player_id: player.id,
        session_id: session1.id,
        status: IntentStatus::Confirmed,
    })
    .await
    .unwrap();
    repo.create(CreateSessionIntentCommand {
        player_id: player.id,
        session_id: session2.id,
        status: IntentStatus::Tentative,
    })
    .await
    .unwrap();

    let found_intents = repo
        .read(GetSessionIntentCommand {
            user_id: Some(player.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(found_intents.len(), 2);
}

#[sqlx::test]
async fn test_get_all_session_intents(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_user(OTHER_PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let player1 = env.seeded.users.get(PLAYER_ID).unwrap();
    let player2 = env.seeded.users.get(OTHER_PLAYER_ID).unwrap();
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();

    repo.create(CreateSessionIntentCommand { player_id: player1.id, session_id: session.id, status: IntentStatus::Confirmed }).await.unwrap();
    repo.create(CreateSessionIntentCommand { player_id: player2.id, session_id: session.id, status: IntentStatus::Tentative }).await.unwrap();

    let all_intents = repo.read(GetSessionIntentCommand::default()).await.unwrap();
    assert_eq!(all_intents.len(), 2);
}

#[sqlx::test]
async fn test_update_session_intent_status(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();

    let created_intent = repo
        .create(CreateSessionIntentCommand {
            player_id: player.id,
            session_id: session.id,
            status: IntentStatus::Tentative,
        })
        .await
        .unwrap();

    repo.update(UpdateSessionIntentCommand {
        id: created_intent.id,
        status: Update::Change(IntentStatus::Confirmed),
    })
    .await
    .unwrap();

    let found_intents = repo
        .read(GetSessionIntentCommand {
            id: Some(created_intent.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(found_intents[0].intent_status, IntentStatus::Confirmed);
}

#[sqlx::test]
async fn test_delete_session_intent(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone())
        .with_user(GM_ID)
        .with_user(PLAYER_ID)
        .with_table(TABLE_ID, GM_ID)
        .with_session(SESSION_ID, TABLE_ID)
        .build()
        .await;
    let repo = PostgresSessionIntentRepository::new(pool.clone());
    let player = env.seeded.users.get(PLAYER_ID).unwrap();
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();

    let created_intent = repo
        .create(CreateSessionIntentCommand {
            player_id: player.id,
            session_id: session.id,
            status: IntentStatus::Confirmed,
        })
        .await
        .unwrap();

    let deleted_intent = repo
        .delete(DeleteSessionIntentCommand { id: created_intent.id })
        .await
        .unwrap();

    assert_eq!(deleted_intent.id, created_intent.id);

    let found = repo
        .read(GetSessionIntentCommand {
            id: Some(created_intent.id),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(found.is_empty());
}

#[sqlx::test]
async fn test_delete_session_intent_not_found(pool: PgPool) {
    let repo = PostgresSessionIntentRepository::new(pool);
    let random_id = Uuid::new_v4();
    let result = repo
        .delete(DeleteSessionIntentCommand { id: random_id })
        .await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_session_intent_operations(pool: PgPool) {
    let env = TestEnvironmentBuilder::new(pool.clone()).with_user(GM_ID).with_table(TABLE_ID, GM_ID).with_session(SESSION_ID, TABLE_ID).build().await;
    let session = env.seeded.sessions.get(SESSION_ID).unwrap();
    let user_repo = PostgresUserRepository::new(pool.clone());
    let repo = PostgresSessionIntentRepository::new(pool.clone());

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let user_repo = user_repo.clone();
            let repo = repo.clone();
            let session_id = session.id;
            tokio::spawn(async move {
                let mut cmd = CreateUserCommand { username: format!("player-{}", i), email: format!("player-{}@test.com", i), password: "password".to_string() };
                let user = user_repo.create(&mut cmd).await.unwrap();
                let intent_data = CreateSessionIntentCommand {
                    player_id: user.id,
                    session_id,
                    status: IntentStatus::Tentative,
                };
                repo
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
    let all_intents = repo
        .read(get_command)
        .await
        .expect("Failed to get all session intents");
    assert_eq!(all_intents.len(), 5);
}