use jos::domain::entities::commands::*;
use jos::domain::repositories::UserRepository;
use jos::infrastructure::persistence::postgres::repositories::PostgresUserRepository;
use jos::shared::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
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
            assert!(user.id != Uuid::nil());
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data).await.unwrap();
    let result = repo.create(&mut user_data).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Expected a persistence error for duplicate email"),
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_username(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data1 = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test1@example.com".to_string(),
        password: "password123".to_string(),
    };
    let mut user_data2 = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test2@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data1).await.unwrap();
    let result = repo.create(&mut user_data2).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Expected a persistence error for duplicate username"),
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();
    let found_user = repo.find_by_id(&created_user.id).await.unwrap().unwrap();

    assert_eq!(found_user.id, created_user.id);
    assert_eq!(found_user.username, "testuser");
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let random_id = Uuid::new_v4();
    let found_user = repo.find_by_id(&random_id).await.unwrap();
    assert!(found_user.is_none());
}

#[sqlx::test]
async fn test_find_by_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data).await.unwrap();
    let found_user = repo
        .find_by_email("test@example.com")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_find_by_email_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let found_user = repo.find_by_email("notfound@example.com").await.unwrap();
    assert!(found_user.is_none());
}

#[sqlx::test]
async fn test_get_all_users(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data1 = CreateUserCommand {
        username: "user1".to_string(),
        email: "user1@example.com".to_string(),
        password: "password123".to_string(),
    };
    let mut user_data2 = CreateUserCommand {
        username: "user2".to_string(),
        email: "user2@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&mut user_data1).await.unwrap();
    repo.create(&mut user_data2).await.unwrap();

    let mut command = GetUserCommand::default();
    let all_users = repo.read(&mut command).await.unwrap();
    assert_eq!(all_users.len(), 2);
}

#[sqlx::test]
async fn test_update_user_username(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "original_username".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        username: jos::domain::entities::update::Update::Change("new_username".to_string()),
        ..Default::default()
    };

    let updated_user = repo.update(&mut update_data).await.unwrap();
    assert_eq!(updated_user.username, "new_username");
}

#[sqlx::test]
async fn test_update_user_email(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "original@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        email: jos::domain::entities::update::Update::Change("new@example.com".to_string()),
        ..Default::default()
    };

    let updated_user = repo.update(&mut update_data).await.unwrap();
    assert_eq!(updated_user.email, "new@example.com");
}

#[sqlx::test]
async fn test_update_user_password(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let mut user_data = CreateUserCommand {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "old_password".to_string(),
    };

    let created_user = repo.create(&mut user_data).await.unwrap();

    let mut update_data = UpdateUserCommand {
        user_id: created_user.id,
        password: jos::domain::entities::update::Update::Change("new_password".to_string()),
        ..Default::default()
    };

    let updated_user = repo.update(&mut update_data).await.unwrap();
    assert_ne!(updated_user.password, "old_password");
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
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
}

#[sqlx::test]
async fn test_delete_user_not_found(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());
    let random_id = Uuid::new_v4();
    let mut delete_command = DeleteUserCommand { id: random_id };
    let result = repo.delete(&mut delete_command).await;

    match result {
        Err(Error::Persistence(_)) => {}
        _ => panic!("Unexpected error: {result:?}"),
    }
}

#[sqlx::test]
async fn test_concurrent_user_creation(pool: PgPool) {
    let repo = PostgresUserRepository::new(pool.clone());

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let repo = repo.clone();
            tokio::spawn(async move {
                let mut user_data = CreateUserCommand {
                    username: format!("user{}", i),
                    email: format!("user{}@example.com", i),
                    password: "password123".to_string(),
                };
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

    let mut command = GetUserCommand::default();
    let all_users = repo.read(&mut command).await.unwrap();
    assert_eq!(all_users.len(), 5);
}