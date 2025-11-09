use crate::persistence::postgres::constraint_mapper;
use crate::persistence::postgres::models::UserModel;
use domain::entities::*;
use domain::repositories::UserRepository;
use shared::error::ApplicationError;
use shared::error::Error;
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
    async fn create(&self, user: &CreateUserCommand) -> Result<User, Error> {
        let created_user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users
                (
                id,
                username,
                email,
                password,
                created_at,
                updated_at)
            VALUES
                ($1, $2, $3, $4, NOW(), NOW())
            RETURNING
                *
            "#,
            user.id,
            user.username,
            user.email,
            user.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_user.into())
    }

    async fn update(&self, data: &UpdateUserCommand) -> Result<User, Error> {
        let has_username_update = matches!(data.username, Some(_));
        let has_email_update = matches!(data.email, Some(_));
        let has_password_update = matches!(data.password, Some(_));

        if !has_username_update && !has_email_update && !has_password_update {
            return Err(Error::Application(ApplicationError::InvalidInput {
                message: "No fields to update".to_string(),
            }));
        }

        let username_value = match &data.username {
            Some(username) => Some(username),
            None => None,
        };

        let email_value = match &data.email {
            Some(email) => Some(email),
            None => None,
        };

        let password_value = match &data.password {
            Some(password) => Some(password),
            None => None,
        };

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
            username_value,
            email_value,
            password_value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_user.into())
    }

    async fn read(&self, command: &GetUserCommand) -> Result<Vec<User>, Error> {
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

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
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

    async fn delete(&self, command: &DeleteUserCommand) -> Result<User, Error> {
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

    async fn delete_by_id(&self, id: Uuid) -> Result<(), Error> {
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

    async fn search(&self, query: &str) -> Result<Vec<User>, Error> {
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
