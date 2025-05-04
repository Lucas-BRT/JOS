use crate::domain::type_wraper::TypeWrapped;
use crate::domain::user::NewUser;
use crate::domain::user::User;
use crate::infra::db::postgres::models::user::RowUserRole;
use crate::infra::db::postgres::models::user::UserRow;
use crate::infra::db::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::{PgPool, query};

pub struct PostgresUserRepository {
    pub pool: PgPool,
}

#[allow(warnings)]
#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &NewUser) -> Result<(), String> {
        query!(
            r#"
                INSERT INTO users (username, display_name, email, password_hash)
                VALUES ($1, $2, $3, $4)
            "#,
            user.username.raw(),
            user.display_name.raw(),
            user.email.raw(),
            user.password.raw()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), String> {
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
