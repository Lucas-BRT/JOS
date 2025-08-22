mod utils;

use jos::Error;
use jos::domain::session::SessionRepository;
use jos::domain::session::commands::CreateSessionCommand;
use jos::domain::session_intent::commands::{
    CreateSessionIntentCommand, DeleteSessionIntentCommand, GetSessionIntentCommand,
    UpdateSessionIntentCommand,
};
use jos::domain::session_intent::{IntentStatus, SessionIntentFilter, SessionIntentRepository};
use jos::domain::table::commands::CreateTableCommand;
use jos::domain::table::entity::Visibility;
use jos::domain::table::table_repository::TableRepository;
use jos::domain::utils::update::Update;
use jos::infrastructure::prelude::{PostgresSessionRepository, PostgresTableRepository};
use jos::infrastructure::repositories::error::RepositoryError;
use jos::infrastructure::repositories::session_intent::PostgresSessionIntentRepository;
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber::util;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_session_intent_success(pool: PgPool) {
    let table_repo = PostgresTableRepository::new(Arc::new(pool.clone()));
    let session_repo = PostgresSessionRepository::new(Arc::new(pool.clone()));
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let user = utils::create_user(&pool).await;
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
    let session = session_repo.create(&session_data).await.unwrap();

    let intent_data = CreateSessionIntentCommand::new(user.id, session.id, IntentStatus::Yes);

    let result = session_intent_repo.create(intent_data).await;

    if let Ok(intent) = result {
        assert_eq!(intent.session_id, session.id);
        assert_eq!(intent.user_id, user.id);
        assert_eq!(intent.intent_status, IntentStatus::Yes);
        assert!(intent.id != Uuid::nil());
    } else {
        panic!("Failed to create session intent: {:?}", result.err());
    }
}

#[sqlx::test]
async fn test_update_session_intent_status(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;

    let player1 = utils::create_user(&pool).await;

    let initial_intent =
        utils::create_session_intent(&pool, player1, session, IntentStatus::Yes).await;

    let update_data = UpdateSessionIntentCommand {
        id: initial_intent.id,
        status: Update::Change(IntentStatus::Yes),
    };

    let result = session_intent_repo.update(update_data).await;

    if let Ok(updated_intent) = result {
        assert_eq!(updated_intent.id, initial_intent.id);
        assert_eq!(updated_intent.intent_status, IntentStatus::Yes);
    } else {
        panic!("Failed to update intent status: {:?}", result.err());
    }
}

#[sqlx::test]
async fn test_delete_session_intent_success(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let player1 = utils::create_user(&pool).await;

    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm, game_system).await;

    let session = utils::create_session(&pool, table).await;

    let intent_to_delete =
        utils::create_session_intent(&pool, player1, session, IntentStatus::No).await;

    let delete_command = DeleteSessionIntentCommand::new(intent_to_delete.id);
    let result = session_intent_repo.delete(delete_command).await;

    match result {
        Ok(session_intent) => {
            assert_eq!(session_intent.id, intent_to_delete.id);
            assert_eq!(session_intent.intent_status, IntentStatus::No);

            let filters = SessionIntentFilter::default().with_id(session_intent.id);

            let get_command = GetSessionIntentCommand {
                filters,
                ..Default::default()
            };
            let found_intent = session_intent_repo.get(get_command).await.unwrap();
            assert!(found_intent.is_empty());
        }
        Err(err) => panic!("Failed to delete session intent: {:?}", err),
    }
}

#[sqlx::test]
async fn test_delete_session_intent_not_found(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));
    let random_id = Uuid::new_v4();
    let delete_command = DeleteSessionIntentCommand::new(random_id);

    let result = session_intent_repo.delete(delete_command).await;

    match result {
        Err(Error::Repository(RepositoryError::NotFound)) => (),
        _ => panic!("Expected NotFound error, but got {:?}", result),
    }
}

#[sqlx::test]
async fn test_get_session_intents_by_session_id(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;

    let mut players = Vec::with_capacity(5);
    for _ in 0..5 {
        players.push(utils::create_user(&pool).await);
    }

    let table = utils::create_table(&pool, gm.clone(), game_system).await;

    let session = utils::create_session(&pool, table.clone()).await;
    let other_session = utils::create_session(&pool, table.clone()).await;

    // Create 3 intents for the main session
    for i in 0..3 {
        utils::create_session_intent(
            &pool,
            players[i].clone(),
            session.clone(),
            IntentStatus::Maybe,
        )
        .await;
    }

    // Create 2 intents for another session
    for i in 0..2 {
        utils::create_session_intent(
            &pool,
            players[i].clone(),
            other_session.clone(),
            IntentStatus::Maybe,
        )
        .await;
    }

    let filters = SessionIntentFilter::default().with_session_id(session.id);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);

    let intents = session_intent_repo.get(get_command).await.unwrap();
    assert_eq!(intents.len(), 3);
    assert!(intents.iter().all(|i| i.session_id == session.id));
}

#[sqlx::test]
async fn test_get_session_intents_with_filters_by_status(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let mut players = Vec::with_capacity(5);
    for _ in 0..5 {
        players.push(utils::create_user(&pool).await);
    }
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;

    utils::create_session_intent(
        &pool,
        players[0].clone(),
        session.clone(),
        IntentStatus::Maybe,
    )
    .await;
    utils::create_session_intent(
        &pool,
        players[1].clone(),
        session.clone(),
        IntentStatus::Yes,
    )
    .await;
    utils::create_session_intent(&pool, players[2].clone(), session.clone(), IntentStatus::No)
        .await;

    let filters = SessionIntentFilter::default().with_intent_status(IntentStatus::Yes);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);

    let accepted_intents = session_intent_repo.get(get_command).await.unwrap();
    assert_eq!(accepted_intents.len(), 1);
    assert_eq!(accepted_intents[0].intent_status, IntentStatus::Yes);
}

#[sqlx::test]
async fn test_create_session_intent_duplicate_user_session(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;
    let player = utils::create_user(&pool).await;

    // Create first intent
    let first_intent =
        utils::create_session_intent(&pool, player.clone(), session.clone(), IntentStatus::Yes)
            .await;

    // Try to create duplicate intent for same user and session
    let duplicate_intent_data =
        CreateSessionIntentCommand::new(player.id, session.id, IntentStatus::Maybe);

    let result = session_intent_repo.create(duplicate_intent_data).await;

    // Should handle duplicate appropriately (either error or update existing)
    match result {
        Err(Error::Repository(RepositoryError::UserSessionIntentAlreadyExists)) => {
            // Expected behavior - duplicate not allowed
        }
        Ok(intent) => {
            // Alternative behavior - updates existing intent
            assert_eq!(intent.user_id, player.id);
            assert_eq!(intent.session_id, session.id);
        }
        _ => panic!(
            "Unexpected result for duplicate intent creation: {:?}",
            result
        ),
    }
}

#[sqlx::test]
async fn test_get_session_intents_by_user_id(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let player = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;

    // Create multiple sessions
    let session1 = utils::create_session(&pool, table.clone()).await;
    let session2 = utils::create_session(&pool, table.clone()).await;
    let session3 = utils::create_session(&pool, table.clone()).await;

    // Create intents for the same user across different sessions
    utils::create_session_intent(&pool, player.clone(), session1.clone(), IntentStatus::Yes).await;
    utils::create_session_intent(&pool, player.clone(), session2.clone(), IntentStatus::Maybe)
        .await;
    utils::create_session_intent(&pool, player.clone(), session3.clone(), IntentStatus::No).await;

    // Create intent for different user to ensure filtering works
    let other_player = utils::create_user(&pool).await;
    utils::create_session_intent(&pool, other_player, session1.clone(), IntentStatus::Yes).await;

    let filters = SessionIntentFilter::default().with_user_id(player.id);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);

    let user_intents = session_intent_repo.get(get_command).await.unwrap();
    assert_eq!(user_intents.len(), 3);
    assert!(user_intents.iter().all(|i| i.user_id == player.id));
}

#[sqlx::test]
async fn test_get_session_intents_with_multiple_filters(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;

    let player1 = utils::create_user(&pool).await;
    let player2 = utils::create_user(&pool).await;

    // Create intents with different statuses
    utils::create_session_intent(&pool, player1.clone(), session.clone(), IntentStatus::Yes).await;
    utils::create_session_intent(&pool, player2.clone(), session.clone(), IntentStatus::No).await;

    // Filter by both session_id and intent_status
    let filters = SessionIntentFilter::default()
        .with_session_id(session.id)
        .with_intent_status(IntentStatus::Yes);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);

    let filtered_intents = session_intent_repo.get(get_command).await.unwrap();
    assert_eq!(filtered_intents.len(), 1);
    assert_eq!(filtered_intents[0].session_id, session.id);
    assert_eq!(filtered_intents[0].intent_status, IntentStatus::Yes);
    assert_eq!(filtered_intents[0].user_id, player1.id);
}

#[sqlx::test]
async fn test_update_session_intent_with_no_change(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;
    let player = utils::create_user(&pool).await;

    let initial_intent =
        utils::create_session_intent(&pool, player, session, IntentStatus::Maybe).await;

    // Update with NoChange
    let update_data = UpdateSessionIntentCommand {
        id: initial_intent.id,
        status: Update::Keep,
    };

    let result = session_intent_repo.update(update_data).await;

    if let Ok(updated_intent) = result {
        assert_eq!(updated_intent.id, initial_intent.id);
        assert_eq!(updated_intent.intent_status, initial_intent.intent_status);
    } else {
        panic!("Failed to update intent with NoChange: {:?}", result.err());
    }
}

#[sqlx::test]
async fn test_update_session_intent_not_found(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));
    let random_id = Uuid::new_v4();

    let update_data = UpdateSessionIntentCommand {
        id: random_id,
        status: Update::Change(IntentStatus::Yes),
    };

    let result = session_intent_repo.update(update_data).await;

    match result {
        Err(Error::Repository(RepositoryError::NotFound)) => (),
        _ => panic!("Expected NotFound error for update, but got {:?}", result),
    }
}

#[sqlx::test]
async fn test_get_session_intents_empty_result(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));
    let random_session_id = Uuid::new_v4();

    let filters = SessionIntentFilter::default().with_session_id(random_session_id);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);

    let intents = session_intent_repo.get(get_command).await.unwrap();
    assert!(intents.is_empty());
}

#[sqlx::test]
async fn test_create_session_intent_all_status_types(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;

    let statuses = [IntentStatus::Yes, IntentStatus::No, IntentStatus::Maybe];

    for (i, status) in statuses.iter().enumerate() {
        let player = utils::create_user(&pool).await;
        let intent_data = CreateSessionIntentCommand::new(player.id, session.id, *status);

        let result = session_intent_repo.create(intent_data).await;

        if let Ok(intent) = result {
            assert_eq!(intent.session_id, session.id);
            assert_eq!(intent.user_id, player.id);
            assert_eq!(intent.intent_status, *status);
            assert!(intent.id != Uuid::nil());
        } else {
            panic!(
                "Failed to create session intent with status {:?}: {:?}",
                status,
                result.err()
            );
        }
    }

    // Verify all intents were created
    let filters = SessionIntentFilter::default().with_session_id(session.id);
    let get_command = GetSessionIntentCommand::default().with_filters(filters);
    let all_intents = session_intent_repo.get(get_command).await.unwrap();
    assert_eq!(all_intents.len(), 3);
}

#[sqlx::test]
async fn test_update_session_intent_status_transitions(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;
    let player = utils::create_user(&pool).await;

    // Create initial intent with Maybe status
    let initial_intent =
        utils::create_session_intent(&pool, player, session, IntentStatus::Maybe).await;

    // Test transition from Maybe to Yes
    let update_to_yes = UpdateSessionIntentCommand {
        id: initial_intent.id,
        status: Update::Change(IntentStatus::Yes),
    };

    let yes_result = session_intent_repo.update(update_to_yes).await.unwrap();
    assert_eq!(yes_result.intent_status, IntentStatus::Yes);

    // Test transition from Yes to No
    let update_to_no = UpdateSessionIntentCommand {
        id: initial_intent.id,
        status: Update::Change(IntentStatus::No),
    };

    let no_result = session_intent_repo.update(update_to_no).await.unwrap();
    assert_eq!(no_result.intent_status, IntentStatus::No);

    // Test transition from No back to Maybe
    let update_to_maybe = UpdateSessionIntentCommand {
        id: initial_intent.id,
        status: Update::Change(IntentStatus::Maybe),
    };

    let maybe_result = session_intent_repo.update(update_to_maybe).await.unwrap();
    assert_eq!(maybe_result.intent_status, IntentStatus::Maybe);
}

#[sqlx::test]
async fn test_get_session_intents_with_limit_and_offset(pool: PgPool) {
    let session_intent_repo = PostgresSessionIntentRepository::new(Arc::new(pool.clone()));

    let gm = utils::create_user(&pool).await;
    let game_system = utils::create_game_system(&pool).await;
    let table = utils::create_table(&pool, gm.clone(), game_system).await;
    let session = utils::create_session(&pool, table.clone()).await;

    // Create 5 intents
    let mut players = Vec::new();
    for _ in 0..5 {
        let player = utils::create_user(&pool).await;
        utils::create_session_intent(&pool, player.clone(), session.clone(), IntentStatus::Yes)
            .await;
        players.push(player);
    }

    let filters = SessionIntentFilter::default().with_session_id(session.id);

    // Test with limit
    let get_command_limit = GetSessionIntentCommand::default()
        .with_filters(filters.clone())
        .with_limit(3);

    let limited_intents = session_intent_repo.get(get_command_limit).await.unwrap();
    assert_eq!(limited_intents.len(), 3);

    // Test with limit and offset
    let get_command_offset = GetSessionIntentCommand::default()
        .with_filters(filters)
        .with_limit(2)
        .with_offset(2);

    let offset_intents = session_intent_repo.get(get_command_offset).await.unwrap();
    assert_eq!(offset_intents.len(), 2);
}
