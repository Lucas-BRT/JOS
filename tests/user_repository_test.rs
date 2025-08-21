use jos::Error;
use jos::domain::user::{
    commands::{CreateUserCommand, UpdateUserCommand},
    role::Role,
    search_commands::UserFilters,
    user_repository::UserRepository as UserRepositoryTrait,
};
use jos::domain::utils::update::Update;
use jos::infrastructure::repositories::error::RepositoryError;
use jos::infrastructure::repositories::user::PostgresUserRepository;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let result = repo.create(&user_data).await;

    match result {
        Ok(user) => {
            assert_eq!(user.username, "testuser");
            assert_eq!(user.email, "test@example.com");
            assert_eq!(user.role, Role::User);
            assert!(user.id != uuid::Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_username_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    repo.create(&user_data)
        .await
        .expect("Failed to create first user");

    let user_data2 = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test2@example.com".into(),
        "password123".into(),
    );

    let result = repo.create(&user_data2).await;

    assert!(result.is_err());

    match result {
        Err(Error::Repository(RepositoryError::UsernameAlreadyTaken)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_email_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data1 = CreateUserCommand::new(
        "testuser1".into(),
        "testuser1".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let user_data2 = CreateUserCommand::new(
        "testuser2".into(),
        "testuser2".into(),
        "test@example.com".into(),
        "password456".into(),
    );

    repo.create(&user_data1)
        .await
        .expect("Failed to create first user");

    let result = repo.create(&user_data2).await;

    match result {
        Err(Error::Repository(RepositoryError::EmailAlreadyTaken)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

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
    if let Err(Error::Repository(RepositoryError::UserNotFound(id))) = result {
        assert_eq!(id, random_id.to_string());
    } else {
        panic!("Expected UserNotFound error");
    }
}

#[sqlx::test]
async fn test_find_by_username(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

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

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_email("test@example.com").await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.username, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_get_all_users(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data1 = CreateUserCommand::new(
        "testuser1".into(),
        "testuser1".into(),
        "test1@example.com".into(),
        "password123".into(),
    );

    let user_data2 = CreateUserCommand::new(
        "testuser2".into(),
        "testuser2".into(),
        "test2@example.com".into(),
        "password456".into(),
    );

    repo.create(&user_data1).await.unwrap();
    repo.create(&user_data2).await.unwrap();

    let all_users = repo.get_all(&UserFilters::default()).await.unwrap();
    assert_eq!(all_users.len(), 2);

    let usernames: Vec<String> = all_users.iter().map(|u| u.username.clone()).collect();
    assert!(usernames.contains(&"testuser1".to_string()));
    assert!(usernames.contains(&"testuser2".to_string()));
}

#[sqlx::test]
async fn test_update_user_displayname(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
        display_name: Update::Change("newusername"),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.display_name, "newusername");
    assert_eq!(updated_user.email, "test@example.com"); // Not changed
}

#[sqlx::test]
async fn test_update_user_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
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

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let created_user = repo.create(&user_data).await.unwrap();

    let update_data = UpdateUserCommand {
        id: created_user.id,
        password: Update::Change("newpassword456"),
        ..Default::default()
    };

    repo.update(&update_data)
        .await
        .expect("Failed to update user");

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.password, "newpassword456");
}

#[sqlx::test]
async fn test_update_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let random_id = uuid::Uuid::new_v4();
    let update_data = UpdateUserCommand {
        id: random_id,
        display_name: Update::Change("newusername"),
        ..Default::default()
    };

    let result = repo.update(&update_data).await;

    match result {
        Err(Error::Repository(RepositoryError::UserNotFound(id))) => {
            assert_eq!(id, random_id.to_string())
        }
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let user_data = CreateUserCommand::new(
        "testuser".into(),
        "testuser".into(),
        "test@example.com".into(),
        "password123".into(),
    );

    let created_user = repo
        .create(&user_data)
        .await
        .expect("Failed to create user");

    let deleted_user = repo
        .delete(&created_user.id)
        .await
        .expect("Failed to delete user");

    assert_eq!(deleted_user.id, created_user.id);
    assert_eq!(deleted_user.username, "testuser");
}

#[sqlx::test]
async fn test_delete_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let random_id = uuid::Uuid::new_v4();
    let result = repo.delete(&random_id).await;

    match result {
        Err(Error::Repository(RepositoryError::UserNotFound(id))) => {
            assert_eq!(id, random_id.to_string())
        }
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_user_operations(pool: PgPool) {
    let repo = PostgresUserRepository::new(Arc::new(pool));

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let repo = repo.clone();
            let user_data = CreateUserCommand::new(
                format!("testuser{i}"),
                format!("testuser{i}"),
                format!("test{i}@example.com"),
                format!("password{i}"),
            );
            tokio::spawn(async move {
                repo.create(&user_data)
                    .await
                    .expect("Failed to create user")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let all_users = repo
        .get_all(&UserFilters::default())
        .await
        .expect("Failed to get all users");
    assert_eq!(all_users.len(), 5);
}
