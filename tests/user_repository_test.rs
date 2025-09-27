use jos::Error;
use jos::adapters::outbound::postgres::RepositoryError;
use jos::adapters::outbound::postgres::repositories::PostgresUserRepository;
use jos::domain::entities::commands::{
    CreateUserCommand, DeleteUserCommand, GetUserCommand, UpdateUserCommand,
};
use jos::domain::entities::update::Update;
use jos::domain::repositories::UserRepository;
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let result = repo.create(&mut user_data).await;

    match result {
        Ok(user) => {
            assert_eq!(user.username, "testuser");
            assert_eq!(user.email, "test@example.com");
            assert!(user.id != uuid::Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_username_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data)
        .await
        .expect("Failed to create first user");

    let mut user_data2 = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test2@example.com".to_string(),
        password: "password123".to_string(),
    };

    let result = repo.create(&mut user_data2).await;

    assert!(result.is_err());

    // result.unwrap();

    match result {
        Err(Error::Persistence(RepositoryError::UsernameAlreadyTaken)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_email_should_fail(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data1 = CreateUserCommand {
        username: "testuser1".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let mut user_data2 = CreateUserCommand {
        username: "testuser2".to_string(),
        email: "test@example.com".to_string(),
        password: "password456".to_string(),
    };

    repo.create(&mut user_data1)
        .await
        .expect("Failed to create first user");

    let result = repo.create(&mut user_data2).await;

    match result {
        Err(Error::Persistence(RepositoryError::EmailAlreadyTaken)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();
    let found_user = repo.find_by_id(&created_user.id).await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    if let Some(user) = found_user {
        assert_eq!(user.id, created_user.id);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    } else {
        panic!("User not found");
    }
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let random_id = uuid::Uuid::new_v4();
    let result = repo
        .find_by_id(&random_id)
        .await
        .expect("Failed to find user by id");

    assert!(result.is_none());
}

#[sqlx::test]
async fn test_find_by_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data).await.unwrap();
    let found_user = repo.find_by_email("test@example.com").await;

    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    if let Some(user) = found_user {
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    } else {
        panic!("User not found");
    }
}

#[sqlx::test]
async fn test_get_all_users(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data1 = CreateUserCommand {
        username: "testuser1".to_string(),
        email: "test1@example.com".to_string(),
        password: "password123".to_string(),
    };

    let mut user_data2 = CreateUserCommand {
        username: "testuser2".to_string(),
        email: "test2@example.com".to_string(),
        password: "password456".to_string(),
    };

    repo.create(&mut user_data1).await.unwrap();
    repo.create(&mut user_data2).await.unwrap();

    let mut get_command = GetUserCommand::default();
    let all_users = repo.read(&mut get_command).await.unwrap();
    assert_eq!(all_users.len(), 2);

    let usernames: Vec<String> = all_users.iter().map(|u| u.username.clone()).collect();
    assert!(usernames.contains(&"testuser1".to_string()));
    assert!(usernames.contains(&"testuser2".to_string()));
}

#[sqlx::test]
async fn test_update_user_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        email: Update::Change("newemail@example.com".to_string()),
        ..Default::default()
    };

    let result = repo.update(&mut update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    if let Some(user) = updated_user {
        assert_eq!(user.username, "testuser"); // Not changed
        assert_eq!(user.email, "newemail@example.com");
    } else {
        panic!("User not found");
    }
}

#[sqlx::test]
async fn test_update_user_password(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        password: Update::Change("newpassword456".to_string()),
        ..Default::default()
    };

    repo.update(&mut update_data)
        .await
        .expect("Failed to update user");

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    if let Some(user) = updated_user {
        assert_eq!(user.password, "newpassword456");
    } else {
        panic!("User not found");
    }
}

#[sqlx::test]
async fn test_update_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        email: Update::Change("newusername".to_string()),
        ..Default::default()
    };

    let result = repo.update(&mut update_data).await;

    match result {
        Ok(user) => {
            assert_eq!(user.id, created_user.id);
        }
        Err(err) => {
            panic!("Unexpected error: {err:?}");
        }
    }
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo
        .create(&mut user_data)
        .await
        .expect("Failed to create user");

    let mut delete_command = DeleteUserCommand {
        id: created_user.id,
    };
    let deleted_user = repo
        .delete(&mut delete_command)
        .await
        .expect("Failed to delete user");

    assert_eq!(deleted_user.id, created_user.id);
    assert_eq!(deleted_user.username, "testuser");
}

#[sqlx::test]
async fn test_delete_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let random_id = uuid::Uuid::new_v4();
    let mut delete_command = DeleteUserCommand { id: random_id };
    let result = repo.delete(&mut delete_command).await;

    match result {
        Err(Error::Persistence(RepositoryError::NotFound)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_user_operations(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool);

    let users_amount = 500;

    let handles: Vec<_> = (0..users_amount)
        .map(|i| {
            let repo = repo.clone();
            let mut user_data = CreateUserCommand {
                username: format!("testuser{i}"),
                email: format!("test{i}@example.com"),
                password: format!("password{i}"),
            };
            tokio::spawn(async move {
                repo.create(&mut user_data)
                    .await
                    .expect("Failed to create user")
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok());
    }

    let mut get_command = GetUserCommand::default();
    let all_users = repo
        .read(&mut get_command)
        .await
        .expect("Failed to get all users");
    assert_eq!(all_users.len(), users_amount);

    let usernames: Vec<String> = all_users.iter().map(|u| u.username.clone()).collect();
    assert!(usernames.contains(&"testuser0".to_string()));
    assert!(usernames.contains(&"testuser1".to_string()));
    assert!(usernames.contains(&"testuser2".to_string()));
    assert!(usernames.contains(&"testuser3".to_string()));
    assert!(usernames.contains(&"testuser4".to_string()));
}
