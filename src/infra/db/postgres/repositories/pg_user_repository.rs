use std::fmt::format;

use crate::domain::type_wraper::TypeWrapped;
use crate::domain::user::NewUser;
use crate::domain::user::User;
use crate::infra::db::postgres::models::user::RowUserRole;
use crate::infra::db::postgres::models::user::UserRow;
use crate::infra::db::repositories::user_repository::UserRepository;
use sqlx::query_as;
use sqlx::{PgPool, query};
use uuid::Uuid;

pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &NewUser) -> Result<String, String> {
        let password_hash = user
            .password
            .hash()
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        query!(
            r#"
                INSERT INTO users (username, display_name, email, password_hash)
                VALUES ($1, $2, $3, $4)
            "#,
            user.username.raw(),
            user.display_name.raw(),
            user.email.raw(),
            password_hash.raw()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        Ok(user.username.raw())
    }

    async fn update(&self, user: &User) -> Result<(), String> {
        query!(
            r#"
            UPDATE users
                SET
                    username = $1,
                    display_name = $2,
                    email = $3,
                    password_hash = $4
                WHERE id = $5
            "#,
            user.username().raw(),
            user.display_name().raw(),
            user.email().raw(),
            user.password_hash().raw(),
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

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<UserRow>, String> {
        Err("".to_string())
    }

    async fn find_by_username(&self, username: &str) -> Result<UserRow, String> {
        Err("".to_string())
    }
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
