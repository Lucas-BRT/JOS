use super::PostgresUserRepository;
use crate::domain::user::dtos::NewUser;
use crate::domain::user::entity::User;
use crate::domain::user::user_repository::UserRepository;
use crate::domain::utils::type_wraper::TypeWrapped;
use crate::infrastructure::persistance::postgres::models::user::RowUserRole;
use crate::infrastructure::persistance::postgres::models::user::UserRow;
use crate::prelude::AppResult;
use async_trait::async_trait;
use sqlx::query_scalar;

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &NewUser) -> AppResult<String> {
        let response = query_scalar!(
            r#"
                INSERT INTO users (username, display_name, email, password_hash)
                VALUES ($1, $2, $3, $4)
                RETURNING username
            "#,
            user.username.raw(),
            user.display_name.raw(),
            user.email.raw(),
            user.password.raw()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(response)
    }

    async fn get_all(&self) -> AppResult<Vec<User>> {
        let rows = sqlx::query_as!(
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
        .await?;

        Ok(Vec::new())
    }

    async fn find_by_username(&self, name: &str) -> AppResult<Option<User>> {
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
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(None)
    }
}
