use crate::domain::type_wraper::TypeWrapped;
use crate::domain::user::NewUser;
use crate::domain::user::User;
use crate::domain::user::ValidatedNewUser;
use crate::error::Error;
use crate::infra::db::postgres::models::user::RowUserRole;
use crate::infra::db::postgres::models::user::UserRow;
use crate::infra::db::repositories::user_repository::UserRepository;
use crate::prelude::AppResult;
use sqlx::query_scalar;
use sqlx::{PgPool, query};

pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &NewUser) -> AppResult<String> {
        let validated_user = ValidatedNewUser::try_from(user.clone())?;

        query_scalar!(
            r#"
                INSERT INTO users (username, display_name, email, password_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING id
            "#,
            validated_user.username.raw(),
            validated_user.display_name.raw(),
            validated_user.email.raw(),
            validated_user.password_hash.raw()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database(e.as_database_error().unwrap().code().unwrap().to_string()))?;

        Ok(validated_user.username.raw())
    }

    async fn update(&self, user: &User) -> Result<(), String> {
        query!(
            r#"
            UPDATE users
                SET
                    username = $1,
                    display_name = $2,
                    email = $3
                WHERE id = $4
            "#,
            user.username().raw(),
            user.display_name().raw(),
            user.email().raw(),
            user.id()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update user: {}", e))?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<UserRow>, String> {
        match sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                id,
                username,
                display_name,
                email,
                password_hash,
                user_role as "user_role: RowUserRole",
                created_at as "created_at: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(rows) => Ok(rows),
            Err(e) => Err(format!("Failed to fetch users: {}", e)),
        }
    }

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<UserRow, String> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    id,
                    username,
                    display_name,
                    email,
                    password_hash,
                    user_role as "user_role: RowUserRole",
                    created_at as "created_at: chrono::DateTime<chrono::Utc>",
                    updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
                FROM users WHERE id = ($1)
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to find user: {}", e.to_string()))?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<UserRow, String> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    id,
                    username,
                    display_name,
                    email,
                    password_hash,
                    user_role as "user_role: RowUserRole",
                    created_at as "created_at: chrono::DateTime<chrono::Utc>",
                    updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
                FROM users
                WHERE username = ($1)
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to find user: {}", e.to_string()))?;

        Ok(user)
    }
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
