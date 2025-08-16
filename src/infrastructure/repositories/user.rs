use crate::Result;
use crate::domain::user::{
    commands::{CreateUserCommand, UpdateUserCommand},
    entity::User,
    search_commands::UserFilters,
    user_repository::UserRepository,
};
use crate::domain::utils::update::Update;
use crate::infrastructure::entities::t_users::Model as UserModel;
use crate::infrastructure::repositories::{constraint_mapper, error::RepositoryError};
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::infrastructure::entities::enums::ERoles;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let created_user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO t_users
                (id,
                username,
                display_name,
                email,
                password_hash,
                role,
                created_at,
                updated_at)
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            "#,
            id,
            &user.username,
            &user.display_name,
            &user.email,
            &user.password,
            ERoles::User as _,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_user.into())
    }

    async fn update(&self, data: &UpdateUserCommand) -> Result<()> {
        let existing_user = sqlx::query_as!(
            UserModel,
            r#"SELECT 
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            FROM t_users 
            WHERE id = $1"#,
            data.id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        if existing_user.is_none() {
            return Err(RepositoryError::UserNotFound.into());
        }

        if let Update::Change(name) = &data.display_name {
            let existing_name = sqlx::query_as!(
                UserModel,
                r#"SELECT 
                    id,
                    username,
                    display_name,
                    email,
                    password_hash,
                    role as "role: ERoles",
                    created_at,
                    updated_at
                FROM t_users 
                    WHERE display_name = $1 AND id != $2"#,
                name,
                data.id
            )
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;

            if existing_name.is_some() {
                return Err(RepositoryError::UsernameAlreadyTaken.into());
            }

            sqlx::query!(
                r#"UPDATE t_users 
                    SET display_name = $1, 
                    updated_at = $2 
                    WHERE id = $3"#,
                name,
                Utc::now(),
                data.id
            )
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;
        }

        if let Update::Change(email) = &data.email {
            let existing_email = sqlx::query_as!(
                UserModel,
                r#"SELECT 
                    id,
                    username,
                    display_name,
                    email,
                    password_hash,
                    role as "role: ERoles",
                    created_at,
                    updated_at
                FROM t_users 
                    WHERE email = $1 AND id != $2"#,
                email,
                data.id
            )
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;

            if existing_email.is_some() {
                return Err(RepositoryError::EmailAlreadyTaken.into());
            }

            sqlx::query!(
                r#"UPDATE t_users 
                    SET email = $1, 
                    updated_at = $2 
                    WHERE id = $3"#,
                email,
                Utc::now(),
                data.id
            )
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;
        }

        if let Update::Change(password) = &data.password {
            sqlx::query!(
                r#"UPDATE t_users 
                    SET password_hash = $1, 
                    updated_at = $2 
                    WHERE id = $3"#,
                password,
                Utc::now(),
                data.id
            )
            .execute(self.pool.as_ref())
            .await
            .map_err(RepositoryError::DatabaseError)?;
        }

        Ok(())
    }

    async fn get_all(&self, filters: &UserFilters) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            FROM t_users
            "#
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(users.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_username(&self, username: &str) -> Result<User> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            FROM t_users
            WHERE username = $1
            LIMIT 1"#,
            username
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            FROM t_users 
            WHERE id = $1"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password_hash,
                role as "role: ERoles",
                created_at,
                updated_at
            FROM t_users
            WHERE email = $1"#,
            email
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn delete(&self, user_id: &Uuid) -> Result<User> {
        let user = self.find_by_id(user_id).await?;

        sqlx::query(
            r#"
            DELETE FROM t_users 
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(user)
    }
}
