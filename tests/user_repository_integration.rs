use jos::domain::user::user_repository::UserRepository;
use jos::infrastructure::persistance::postgres::repositories::error::RepositoryError;
use jos::infrastructure::persistance::postgres::repositories::user::PostgresUserRepository;
use jos::{Error, domain::user::dtos::CreateUserCommand};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

mod create {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_user_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let new_user_data = CreateUserCommand {
            name: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "strongpassword123".to_string(),
            confirm_password: "strongpassword123".to_string(),
        };

        let result = user_repo.create(&new_user_data).await;

        assert!(result.is_ok());
        let created_user = result.unwrap();

        assert_eq!(created_user.name, new_user_data.name);
        assert_eq!(created_user.email, new_user_data.email);
        assert_ne!(created_user.password_hash, new_user_data.password);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_user_fails_on_duplicate_email(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let user_data1 = CreateUserCommand {
            name: "user1".to_string(),
            email: "duplicate@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let user_data2 = CreateUserCommand {
            name: "user2".to_string(),
            email: "duplicate@example.com".to_string(),
            password: "password456".to_string(),
            confirm_password: "password456".to_string(),
        };

        let first_result = user_repo.create(&user_data1).await;
        assert!(first_result.is_ok());

        let second_result = user_repo.create(&user_data2).await;

        assert!(second_result.is_err());
        match second_result.err().unwrap() {
            Error::Repository(RepositoryError::EmailAlreadyTaken(email)) => {
                assert_eq!(email, user_data2.email);
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_user_fails_on_duplicate_username(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let user_data1 = CreateUserCommand {
            name: "duplicate_user".to_string(),
            email: "user1@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let user_data2 = CreateUserCommand {
            name: "duplicate_user".to_string(),
            email: "user2@example.com".to_string(),
            password: "password456".to_string(),
            confirm_password: "password456".to_string(),
        };

        assert!(user_repo.create(&user_data1).await.is_ok());

        let second_result = user_repo.create(&user_data2).await;

        assert!(second_result.is_err());
        match second_result.err().unwrap() {
            Error::Repository(RepositoryError::UsernameAlreadyTaken(name)) => {
                assert_eq!(name, user_data2.name);
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }
}

mod read {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_id_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let new_user_data = CreateUserCommand {
            name: "find_me_by_id".to_string(),
            email: "findme@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let created_user = user_repo.create(&new_user_data).await.unwrap();

        let result = user_repo.find_by_id(&created_user.id).await;

        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.name, new_user_data.name);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_id_not_found(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let non_existent_id = Uuid::new_v4();

        let result = user_repo.find_by_id(&non_existent_id).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            Error::Repository(RepositoryError::UserNotFound) => {
                // Success, the expected error was returned.
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_email_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let new_user_data = CreateUserCommand {
            name: "find_me_by_email".to_string(),
            email: "findme@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let created_user = user_repo.create(&new_user_data).await.unwrap();

        let result = user_repo.find_by_email(&new_user_data.email).await;

        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.name, new_user_data.name);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_email_not_found(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let non_existent_email = "nonexistent@example.com";

        let result = user_repo.find_by_email(non_existent_email).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            Error::Repository(RepositoryError::UserNotFound) => {
                // Success, the expected error was returned.
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_username_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let new_user_data = CreateUserCommand {
            name: "find_me_by_username".to_string(),
            email: "findme@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let created_user = user_repo.create(&new_user_data).await.unwrap();

        let result = user_repo.find_by_username(&new_user_data.name).await;

        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.name, new_user_data.name);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_find_by_username_not_found(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let non_existent_username = "nonexistent";

        let result = user_repo.find_by_username(non_existent_username).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            Error::Repository(RepositoryError::UserNotFound) => {
                // Success, the expected error was returned.
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }
}

mod update {
    use jos::domain::user::dtos::UpdateUserCommand;

    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_user_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let user_data = CreateUserCommand {
            name: "update_me".to_string(),
            email: "update@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let created_user = user_repo.create(&user_data).await.unwrap();

        let updated_user_data = UpdateUserCommand {
            id: created_user.id,
            name: None,
            email: None,
            password: None,
            bio: None,
            avatar_url: None,
            nickname: Some("updated_nickname".to_string()),
            years_of_experience: Some(5),
        };
        let result = user_repo.update(&updated_user_data).await;

        assert!(result.is_ok());

        let found_user = user_repo.find_by_id(&created_user.id).await.unwrap();

        assert_eq!(
            found_user.nickname,
            Some(updated_user_data.nickname.unwrap())
        );
    }
}

mod delete {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_user_success(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let new_user_data = CreateUserCommand {
            name: "delete_me".to_string(),
            email: "delete@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let created_user = user_repo.create(&new_user_data).await.unwrap();

        let result = user_repo.delete(&created_user.id).await;

        assert!(result.is_ok());

        let deleted_user = user_repo.find_by_id(&created_user.id).await;
        assert!(
            deleted_user
                .is_err_and(|e| matches!(e, Error::Repository(RepositoryError::UserNotFound)))
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_user_not_found(pool: PgPool) {
        let user_repo = PostgresUserRepository::new(Arc::new(pool));
        let non_existent_id = Uuid::new_v4();

        let result = user_repo.delete(&non_existent_id).await;

        assert!(result.is_err());
        match result.err().unwrap() {
            Error::Repository(RepositoryError::UserNotFound) => {
                // Success, the expected error was returned.
            }
            err => panic!("Unexpected error: {:?}", err),
        }
    }
}
