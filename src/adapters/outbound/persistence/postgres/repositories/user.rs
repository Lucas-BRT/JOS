use crate::Result;
use crate::domain::user::{
    commands::{CreateUserCommand, UpdateUserCommand},
    entity::User,
    search_commands::UserFilters,
    user_repository::UserRepository,
};
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
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
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

    async fn update(&self, data: &UpdateUserCommand) -> Result<()> {
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
        builder.push_bind(data.id);

        builder.push(" RETURNING *");

        let updated_user = builder
            .build_query_as::<UserModel>()
            .fetch_one(&self.pool)
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_user.into())
    }

    async fn get_all(&self, _filters: &UserFilters) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
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
            "#
        )
        .fetch_all(&self.pool)
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
                password,
                created_at,
                updated_at
            FROM users
            WHERE username = $1
            LIMIT 1"#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound(username.to_string()).into())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<User> {
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
            WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound(id.to_string()).into())
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
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
            WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(|model| model.into())
            .ok_or(RepositoryError::UserNotFound(email.to_string()).into())
    }

    async fn delete(&self, user_id: &Uuid) -> Result<User> {
        let user = self.find_by_id(user_id).await?;

        sqlx::query(
            r#"
            DELETE FROM users 
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(user)
    }
}
