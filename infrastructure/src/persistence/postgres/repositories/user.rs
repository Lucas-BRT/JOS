use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::UserModel;
use domain::entities::*;
use domain::repositories::{Repository, UserRepository};
use shared::Result;
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
impl Repository<User, CreateUserCommand, UpdateUserCommand, GetUserCommand, DeleteUserCommand>
    for PostgresUserRepository
{
    async fn create(&self, user: CreateUserCommand) -> Result<User> {
        let created_user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users
                (id, username, email, password, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, NOW(), NOW())
            RETURNING
                id, username, email, password, created_at, updated_at
            "#,
            user.id,
            &user.username,
            &user.email,
            &user.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_user.into())
    }

    async fn update(&self, data: UpdateUserCommand) -> Result<User> {
        let updated_user = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET
                username = COALESCE($2, username),
                email = COALESCE($3, email),
                password = COALESCE($4, password),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
            data.user_id,
            data.username.as_deref(),
            data.email.as_deref(),
            data.password.as_deref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_user.into())
    }

    async fn read(&self, command: GetUserCommand) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, username, email, password, created_at, updated_at
            FROM users
            WHERE ($1::uuid IS NULL OR id = $1)
              AND ($2::text IS NULL OR username = $2)
              AND ($3::text IS NULL OR email = $3)
            "#,
            command.id,
            command.username.as_deref(),
            command.email.as_deref()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(users.into_iter().map(|model| model.into()).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT id, username, email, password, created_at, updated_at
                FROM users
                WHERE id = $1
            "#,
            &id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(user.map(|model| model.into()))
    }

    async fn delete(&self, command: DeleteUserCommand) -> Result<User> {
        let user = sqlx::query_as!(
            UserModel,
            r#"DELETE FROM users
            WHERE id = $1
            RETURNING
                id, username, email, password, created_at, updated_at
            "#,
            &command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(user.into())
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
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

    async fn delete_by_id(&self, id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<User>> {
        let search_pattern = format!("%{}%", query);
        let users = sqlx::query_as!(
            UserModel,
            r#"SELECT id, username, email, password, created_at, updated_at
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
