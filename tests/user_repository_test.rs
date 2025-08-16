use jos::domain::user::{
    commands::{CreateUserCommand, UpdateUserCommand}, role::Role, search_commands::UserFilters, user_repository::UserRepository as UserRepositoryTrait
};
use jos::domain::utils::update::Update;
use jos::infrastructure::entities::enums::ERoles;
use jos::infrastructure::repositories::error::RepositoryError;
use jos::infrastructure::repositories::user::PostgresUserRepository;
use sqlx::PgPool;
use std::sync::Arc;

fn create_test_user_data(
    username: &str,
    display_name: &str,
    email: &str,
    password: &str,
) -> CreateUserCommand {
    CreateUserCommand {
        username: username.to_string(),
        display_name: display_name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    }
}

#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let result = repo.create(&user_data).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.role, Role::User);
    assert!(user.id != uuid::Uuid::nil());
}

#[sqlx::test]
async fn test_create_user_duplicate_username_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    // Create first user
    let result1 = repo.create(&user_data).await;
    assert!(result1.is_ok());

    let user_data2 =
        create_test_user_data("testuser", "testuser", "test2@example.com", "password123");

    // Try to create second user with same username
    let result2 = repo.create(&user_data2).await;

    assert!(result2.is_err());

    match result2 {
        Err(jos::Error::Repository(RepositoryError::UsernameAlreadyTaken)) => (),
        _ => panic!("Unexpected error: {:?}", result2),
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_email_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data1 =
        create_test_user_data("testuser1", "testuser1", "test@example.com", "password123");

    let user_data2 =
        create_test_user_data("testuser2", "testuser2", "test@example.com", "password456");

    // Create first user
    let result1 = repo.create(&user_data1).await;
    assert!(result1.is_ok());

    // Try to create second user with same email
    let result2 = repo.create(&user_data2).await;
    assert!(result2.is_err());

    match result2 {
        Err(jos::Error::Repository(RepositoryError::EmailAlreadyTaken)) => (),
        _ => panic!("Unexpected error: {:?}", result2),
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let created_user = repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_id(&created_user.id).await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, created_user.id);
    assert_eq!(found_user.username, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let random_id = uuid::Uuid::new_v4();
    let result = repo.find_by_id(&random_id).await;

    assert!(result.is_err());
    if let Err(jos::Error::Repository(RepositoryError::UserNotFound)) = result {
        // Expected error
    } else {
        panic!("Expected UserNotFound error");
    }
}

#[sqlx::test]
async fn test_find_by_username(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_username("testuser").await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.username, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_find_by_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_email("test@example.com").await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.username, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_get_all(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data1 =
        create_test_user_data("testuser1", "testuser1", "test1@example.com", "password123");

    let user_data2 =
        create_test_user_data("testuser2", "testuser2", "test2@example.com", "password456");

    repo.create(&user_data1).await.unwrap();
    repo.create(&user_data2).await.unwrap();

    let all_users = repo.get_all(&UserFilters::default()).await.unwrap();
    assert_eq!(all_users.len(), 2);

    let usernames: Vec<String> = all_users.iter().map(|u| u.username.clone()).collect();
    assert!(usernames.contains(&"testuser1".to_string()));
    assert!(usernames.contains(&"testuser2".to_string()));
}

#[sqlx::test]
async fn test_update_user_name(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
        display_name: Update::Change("newusername"),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.username, "newusername");
    assert_eq!(updated_user.email, "test@example.com"); // Not changed
}

#[sqlx::test]
async fn test_update_user_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
        display_name: Update::Keep,
        email: Update::Change("newemail@example.com"),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.username, "testuser"); // Not changed
    assert_eq!(updated_user.email, "newemail@example.com");
}

#[sqlx::test]
async fn test_update_user_password(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
        display_name: Update::Keep,
        email: Update::Keep,
        password: Update::Change("newpassword456"),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.password_hash, "newpassword456");
}

#[sqlx::test]
async fn test_update_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let random_id = uuid::Uuid::new_v4();
    let update_data = UpdateUserCommand {
        id: random_id,
        display_name: Update::Change("newusername"),
        email: Update::Keep,
        password: Update::Keep,
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_err());

    if let Err(jos::Error::Repository(RepositoryError::UserNotFound)) = result {
        // Expected error
    } else {
        panic!("Expected UserNotFound error");
    }
}

#[sqlx::test]
async fn test_update_user_duplicate_username(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    // Create two users
    let user_data1 =
        create_test_user_data("testuser1", "testuser1", "test1@example.com", "password123");

    let user_data2 =
        create_test_user_data("testuser2", "testuser2", "test2@example.com", "password456");

    let user1 = repo.create(&user_data1).await.unwrap();
    repo.create(&user_data2).await.unwrap();

    // Try to update second user with first user's name
    let update_data = UpdateUserCommand {
        id: user1.id,
        display_name: Update::Change("testuser2"),
        email: Update::Keep,
        password: Update::Keep,
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_err());

    match result {
        Err(jos::Error::Repository(RepositoryError::UsernameAlreadyTaken)) => (),
        _ => panic!("Unexpected error: {:?}", result),
    }
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data =
        create_test_user_data("testuser", "testuser", "test@example.com", "password123");

    let created_user = repo.create(&user_data).await.unwrap();

    // Verify that the user exists
    let found_user = repo.find_by_id(&created_user.id).await;
    assert!(found_user.is_ok());

    // Delete the user
    let deleted_user = repo.delete(&created_user.id).await;

    assert!(deleted_user.is_ok());
    let deleted_user = deleted_user.unwrap();
    assert_eq!(deleted_user.id, created_user.id);
    assert_eq!(deleted_user.username, "testuser");

    // Verify that the user does not exist anymore
    let found_user = repo.find_by_id(&created_user.id).await;
    assert!(found_user.is_err());
}

#[sqlx::test]
async fn test_delete_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let random_id = uuid::Uuid::new_v4();
    let result = repo.delete(&random_id).await;

    assert!(result.is_err());
    if let Err(jos::Error::Repository(RepositoryError::UserNotFound)) = result {
        // Expected error
    } else {
        panic!("Expected UserNotFound error");
    }
}

#[sqlx::test]
async fn test_concurrent_user_operations(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    // Create multiple users simultaneously
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let repo = repo.clone();
            let user_data = create_test_user_data(
                &format!("testuser{}", i),
                &format!("testuser{}", i),
                &format!("test{}@example.com", i),
                &format!("password{}", i),
            );
            tokio::spawn(async move { repo.create(&user_data).await })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.unwrap().is_ok());
    }

    // Verify that all users were created
    let all_users = repo.get_all(&UserFilters::default()).await.unwrap();
    assert_eq!(all_users.len(), 5);
}

