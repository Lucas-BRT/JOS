use jos::infrastructure::repositories::user::UserRepository;
use jos::domain::user::{
    dtos::{CreateUserCommand, UpdateUserCommand},
    role::Role,
    user_repository::UserRepository as UserRepositoryTrait,
};
use jos::domain::utils::update::Update;
use jos::infrastructure::entities::enums::ERoles;
use jos::infrastructure::repositories::error::RepositoryError;
use sqlx::PgPool;
use std::sync::Arc;


#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let result = repo.create(&user_data).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.name, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.role, Role::User);
    assert!(user.id != uuid::Uuid::nil());
}

#[sqlx::test]
async fn test_create_user_duplicate_username(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Create first user
    let result1 = repo.create(&user_data).await;
    assert!(result1.is_ok());

    // Try to create second user with same username
    let result2 = repo.create(&user_data).await;
    assert!(result2.is_err());
    
    if let Err(jos::Error::Repository(RepositoryError::UsernameAlreadyTaken(name))) = result2 {
        assert_eq!(name, "testuser");
    } else {
        panic!("Expected UsernameAlreadyTaken error");
    }
}

#[sqlx::test]
async fn test_create_user_duplicate_email(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data1 = CreateUserCommand {
        name: "testuser1".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let user_data2 = CreateUserCommand {
        name: "testuser2".to_string(),
        email: "test@example.com".to_string(),
        password: "password456".to_string(),
    };

    // Create first user
    let result1 = repo.create(&user_data1).await;
    assert!(result1.is_ok());

    // Try to create second user with same email
    let result2 = repo.create(&user_data2).await;
    assert!(result2.is_err());
    
    if let Err(jos::Error::Repository(RepositoryError::EmailAlreadyTaken(email))) = result2 {
        assert_eq!(email, "test@example.com");
    } else {
        panic!("Expected EmailAlreadyTaken error");
    }
}

#[sqlx::test]
async fn test_find_by_id(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_id(&created_user.id).await;
    
    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, created_user.id);
    assert_eq!(found_user.name, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_find_by_id_not_found(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
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
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_username("testuser").await;
    
    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.name, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_find_by_email(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    repo.create(&user_data).await.unwrap();
    let found_user = repo.find_by_email("test@example.com").await;
    
    assert!(found_user.is_ok());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.name, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}

#[sqlx::test]
async fn test_get_all(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data1 = CreateUserCommand {
        name: "testuser1".to_string(),
        email: "test1@example.com".to_string(),
        password: "password123".to_string(),
    };

    let user_data2 = CreateUserCommand {
        name: "testuser2".to_string(),
        email: "test2@example.com".to_string(),
        password: "password456".to_string(),
    };

    repo.create(&user_data1).await.unwrap();
    repo.create(&user_data2).await.unwrap();

    let all_users = repo.get_all().await.unwrap();
    assert_eq!(all_users.len(), 2);

    let usernames: Vec<String> = all_users.iter().map(|u| u.name.clone()).collect();
    assert!(usernames.contains(&"testuser1".to_string()));
    assert!(usernames.contains(&"testuser2".to_string()));
}

#[sqlx::test]
async fn test_update_user_name(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&user_data).await.unwrap();
    
    let update_data = UpdateUserCommand {
        id: created_user.id,
        name: Update::Change("newusername".to_string()),
        email: Update::Keep,
        password: Update::Keep,
        bio: Update::Keep,
        avatar_url: Update::Keep,
        nickname: Update::Keep,
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.name, "newusername");
    assert_eq!(updated_user.email, "test@example.com"); // Not changed
}

#[sqlx::test]
async fn test_update_user_email(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&user_data).await.unwrap();
    
    let update_data = UpdateUserCommand {
        id: created_user.id,
        name: Update::Keep,
        email: Update::Change("newemail@example.com".to_string()),
        password: Update::Keep,
        bio: Update::Keep,
        avatar_url: Update::Keep,
        nickname: Update::Keep,
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.name, "testuser"); // Not changed
    assert_eq!(updated_user.email, "newemail@example.com");
}

#[sqlx::test]
async fn test_update_user_password(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&user_data).await.unwrap();
    
    let update_data = UpdateUserCommand {
        id: created_user.id,
        name: Update::Keep,
        email: Update::Keep,
        password: Update::Change("newpassword456".to_string()),
        bio: Update::Keep,
        avatar_url: Update::Keep,
        nickname: Update::Keep,
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_ok());

    let updated_user = repo.find_by_id(&created_user.id).await.unwrap();
    assert_eq!(updated_user.password_hash, "newpassword456");
}

#[sqlx::test]
async fn test_update_user_not_found(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let random_id = uuid::Uuid::new_v4();
    let update_data = UpdateUserCommand {
        id: random_id,
        name: Update::Change("newusername".to_string()),
        email: Update::Keep,
        password: Update::Keep,
        bio: Update::Keep,
        avatar_url: Update::Keep,
        nickname: Update::Keep,
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
    let repo = UserRepository::new(Arc::new(pool));
    
    // Create two users
    let user_data1 = CreateUserCommand {
        name: "testuser1".to_string(),
        email: "test1@example.com".to_string(),
        password: "password123".to_string(),
    };

    let user_data2 = CreateUserCommand {
        name: "testuser2".to_string(),
        email: "test2@example.com".to_string(),
        password: "password456".to_string(),
    };

    let user1 = repo.create(&user_data1).await.unwrap();
    repo.create(&user_data2).await.unwrap();

    // Try to update second user with first user's name
    let update_data = UpdateUserCommand {
        id: user1.id,
        name: Update::Change("testuser2".to_string()),
        email: Update::Keep,
        password: Update::Keep,
        bio: Update::Keep,
        avatar_url: Update::Keep,
        nickname: Update::Keep,
    };

    let result = repo.update(&update_data).await;
    assert!(result.is_err());
    
    if let Err(jos::Error::Repository(RepositoryError::UsernameAlreadyTaken(name))) = result {
        assert_eq!(name, "testuser2");
    } else {
        panic!("Expected UsernameAlreadyTaken error");
    }
}

#[sqlx::test]
async fn test_delete_user(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    let user_data = CreateUserCommand {
        name: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let created_user = repo.create(&user_data).await.unwrap();
    
    // Verify that the user exists
    let found_user = repo.find_by_id(&created_user.id).await;
    assert!(found_user.is_ok());

    // Delete the user
    let deleted_user = repo.delete(&created_user.id).await.unwrap();
    assert_eq!(deleted_user.id, created_user.id);
    assert_eq!(deleted_user.name, "testuser");

    // Verify that the user does not exist anymore
    let found_user = repo.find_by_id(&created_user.id).await;
    assert!(found_user.is_err());
}

#[sqlx::test]
async fn test_delete_user_not_found(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
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
    let repo = UserRepository::new(Arc::new(pool));
    
    // Create multiple users simultaneously
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let repo = repo.clone();
            let user_data = CreateUserCommand {
                name: format!("testuser{}", i),
                email: format!("test{}@example.com", i),
                password: format!("password{}", i),
            };
            tokio::spawn(async move { repo.create(&user_data).await })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles).await;
    
    for result in results {
        assert!(result.unwrap().is_ok());
    }

    // Verify that all users were created
    let all_users = repo.get_all().await.unwrap();
    assert_eq!(all_users.len(), 5);
}

#[sqlx::test]
async fn test_role_mapping(pool: PgPool) {
    let repo = UserRepository::new(Arc::new(pool));
    
    // Test mapping of roles
    assert_eq!(
        UserRepository::map_role_to_entity(ERoles::Admin),
        Role::Admin
    );
    assert_eq!(
        UserRepository::map_role_to_entity(ERoles::Moderator),
        Role::Moderator
    );
    assert_eq!(
        UserRepository::map_role_to_entity(ERoles::User),
        Role::User
    );

    assert_eq!(
        UserRepository::map_role_to_db(&Role::Admin),
        ERoles::Admin
    );
    assert_eq!(
        UserRepository::map_role_to_db(&Role::Moderator),
        ERoles::Moderator
    );
    assert_eq!(
        UserRepository::map_role_to_db(&Role::User),
        ERoles::User
    );
}
