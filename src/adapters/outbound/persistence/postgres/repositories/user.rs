use crate::adapters::outbound::postgres::constraint_mapper;
use crate::adapters::outbound::postgres::models::UserModel;
use crate::domain::entities::*;
use crate::domain::error::UserDomainError;
use crate::domain::repositories::UserRepository;
use crate::{Error, Result};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &mut CreateUserCommand) -> Result<User> {
        let created_user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users
                (
                username,
                email,
                password)
            VALUES
                ($1, $2, $3)
            RETURNING
                *
            "#,
            &user.username,
            &user.email,
            &user.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_user.into())
    }

    async fn update(&self, data: &mut UpdateUserCommand) -> Result<User> {
        let has_email_update = matches!(data.email, Update::Change(_));
        let has_password_update = matches!(data.password, Update::Change(_));

        if !has_email_update && !has_password_update {
            return Err(Error::Domain(UserDomainError::UserNotFound.into()));
        }

        let email_value = match &data.email {
            Update::Change(email) => Some(email.as_str()),
            Update::Keep => None,
        };

        let password_value = match &data.password {
            Update::Change(password) => Some(password.as_str()),
            Update::Keep => None,
        };

        let updated_user = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users 
            SET 
                email = COALESCE($2, email),
                password = COALESCE($3, password),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            data.user_id,
            email_value,
            password_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_user.into())
    }

    async fn read(&self, command: &mut GetUserCommand) -> Result<Vec<User>> {
        let mut query = sqlx::QueryBuilder::new("SELECT * FROM users");
        let mut conditions = Vec::new();

        if let Some(id) = &command.id {
            conditions.push("id = ");
            query.push_bind(id);
        }

        if let Some(username) = &command.username {
            conditions.push("username = ");
            query.push_bind(username);
        }

        if let Some(email) = &command.email {
            conditions.push("email = ");
            query.push_bind(email);
        }

        if !conditions.is_empty() {
            query.push(" WHERE ");
            for (i, condition) in conditions.iter().enumerate() {
                if i > 0 {
                    query.push(" AND ");
                }
                query.push(condition);
            }
        }

        let users = query
            .build_query_as::<UserModel>()
            .fetch_all(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(users.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                email,
                password,
                created_at,
                updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(user.map(|model| model.into()))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT *
                FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(user.map(|model| model.into()))
    }

    async fn delete(&self, command: &mut DeleteUserCommand) -> Result<User> {
        let user = sqlx::query_as!(
            UserModel,
            r#"DELETE FROM users
            WHERE id = $1
            RETURNING
                *
            "#,
            &command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(user.into())
    }

    async fn search(&self, query: &str) -> Result<Vec<User>> {
        let search_pattern = format!("%{}%", query);
        let users = sqlx::query_as!(
            UserModel,
            r#"SELECT *
                FROM users
                WHERE username ILIKE $1 OR email ILIKE $1
                LIMIT 10
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(users.into_iter().map(|model| model.into()).collect())
    }
}
