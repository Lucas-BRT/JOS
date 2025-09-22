use crate::Result;
use crate::adapters::outbound::postgres::models::UserModel;
use crate::adapters::outbound::postgres::{RepositoryError, constraint_mapper};
use crate::domain::entities::{
    CreateUserCommand, DeleteUserCommand, GetUserCommand, UpdateUserCommand, User,
};
use crate::domain::repositories::UserRepository;
use crate::domain::utils::update::Update;
use sqlx::PgPool;

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
    async fn create(&self, user: CreateUserCommand) -> Result<User> {
        let created_user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users
                (
                username,
                display_name,
                email,
                password)
            VALUES
                ($1, $2, $3, $4)
            RETURNING
                *
            "#,
            &user.username,
            &user.display_name,
            &user.email,
            &user.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_user.into())
    }

    async fn update(&self, data: UpdateUserCommand) -> Result<User> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut separated = builder.separated(", ");

        if let Update::Change(display_name) = &data.display_name {
            separated.push("display_name = ");
            separated.push_bind_unseparated(display_name);
        }

        if let Update::Change(email) = &data.email {
            separated.push("email = ");
            separated.push_bind_unseparated(email);
        }

        if let Update::Change(password) = &data.password {
            separated.push("password = ");
            separated.push_bind_unseparated(password);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(data.user_id);

        builder.push(" RETURNING *");

        let updated_user = builder
            .build_query_as::<UserModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_user.into())
    }

    async fn read(&self, command: GetUserCommand) -> Result<Vec<User>> {
        let mut query = sqlx::QueryBuilder::new(
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password,
                created_at,
                updated_at
            FROM users
            "#,
        );

        let mut conditions = Vec::new();

        if let Some(id) = &command.id {
            conditions.push("id = ");
            query.push_bind(id);
        }

        if let Some(username) = &command.username {
            conditions.push("username = ");
            query.push_bind(username);
        }

        if let Some(display_name) = &command.display_name {
            conditions.push("display_name = ");
            query.push_bind(display_name);
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
            .map_err(RepositoryError::DatabaseError)?;

        Ok(users.into_iter().map(|model| model.into()).collect())
    }

    async fn delete(&self, command: DeleteUserCommand) -> Result<User> {
        // First get the user to return it
        let user = sqlx::query_as!(
            UserModel,
            r#"SELECT
                id,
                username,
                display_name,
                email,
                password,
                created_at,
                updated_at
            FROM users
            WHERE id = $1
            "#,
            &command.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        // Then delete it
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(&command.id)
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(user.into())
    }
}
