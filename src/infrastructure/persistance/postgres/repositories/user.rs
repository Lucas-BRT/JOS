use crate::core::error::AppError;
use crate::domain::error::DomainError;
use crate::domain::user::dtos::NewUser;
use crate::domain::user::entity::User;
use crate::domain::user::user_repository::UserRepository;
use crate::domain::utils::type_wraper::TypeWrapped;
use crate::infrastructure::persistance::postgres::models::user::AccessLevel;
use crate::infrastructure::persistance::postgres::models::user::UserRow;
use crate::prelude::AppResult;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query_scalar;
use std::ops::Deref;
use std::sync::Arc;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl<'a> PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl TryFrom<UserRow> for User {
    type Error = AppError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let username = row
            .username
            .parse()
            .map_err(|e| AppError::Domain(DomainError::User(e)))?;
        let display_name = row
            .display_name
            .parse()
            .map_err(|e| AppError::Domain(DomainError::User(e)))?;
        let email = row
            .email
            .parse()
            .map_err(|e| AppError::Domain(DomainError::User(e)))?;
        let access_level = row.access_level.into();

        Ok(User {
            id: row.id,
            username,
            display_name,
            email,
            access_level,
        })
    }
}

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
        .fetch_one(self.pool.deref())
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
                access_level as "access_level: AccessLevel",
                created_at as "created_at: chrono::DateTime<chrono::Utc>",
                updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            FROM users
            "#
        )
        .fetch_all(self.pool.deref())
        .await?;

        let users = rows
            .into_iter()
            .map(|row| row.try_into())
            .collect::<AppResult<Vec<User>>>()?;

        Ok(users)
    }

    async fn find_by_username(&self, name: &str) -> AppResult<Option<User>> {
        // let user = sqlx::query_as!(
        //     UserRow,
        //     r#"
        //         SELECT
        //             id,
        //             username,
        //             display_name,
        //             email,
        //             password_hash,
        //             user_role as "user_role: RowUserRole",
        //             created_at as "created_at: chrono::DateTime<chrono::Utc>",
        //             updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
        //         FROM users
        //         WHERE username = ($1)
        //     "#,
        //     name
        // )
        // .fetch_optional(self.pool.deref())
        // .await?;

        // Ok(user.map(|row| row.try_into()).transpose()?)

        todo!()
    }
}
