use super::PostgresRepository;
use crate::domain::type_wraper::TypeWrapped;
use crate::domain::user::NewUser;
use crate::domain::user::User;
use crate::domain::user::ValidatedUser;
use crate::infra::db::postgres::models::user::RowUserRole;
use crate::infra::db::postgres::models::user::UserRow;
use crate::infra::db::repositories::user_repository::UserRepository;
use crate::prelude::AppResult;
use sqlx::query;
use sqlx::query_scalar;

impl UserRepository for PostgresRepository {
    async fn create(&self, user: &NewUser) -> AppResult<String> {
        let validated_user = ValidatedUser::try_from(user.clone())?;

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
        .await?;

        Ok(validated_user.username.raw())
    }

    async fn update(&self, user: &User) -> AppResult<()> {
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
        .await?;

        Ok(())
    }

    async fn get_all(&self) -> AppResult<Vec<UserRow>> {
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

        Ok(rows)
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<UserRow>> {
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
