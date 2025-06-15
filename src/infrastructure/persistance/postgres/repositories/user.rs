use crate::Error;
use crate::Result;
use crate::domain::user::dtos::CreateUserCommand;
use crate::domain::user::dtos::UpdateUserCommand;
use crate::domain::user::entity::User;
use crate::domain::user::user_repository::UserRepository;
use crate::domain::utils::update::Update;
use crate::infrastructure::persistance::postgres::models::user::AccessLevelModel;
use crate::infrastructure::persistance::postgres::models::user::Model as UserModel;
use crate::infrastructure::persistance::postgres::repositories::error::RepositoryError;
use crate::utils::password::generate_hash;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::QueryBuilder;
use sqlx::{Error as SqlxError, postgres::PgDatabaseError};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
        let uuid = Uuid::new_v4();

        let password_hash = generate_hash(user.password.clone()).await?;
        let access_level = AccessLevelModel::User;

        let result = sqlx::query_as!(
            UserModel,
            r#"
                INSERT INTO users (
                    id,
                    name,
                    email,
                    password_hash,
                    access_level
                )
                VALUES ($1, $2, $3, $4, $5)
                RETURNING
                    id,
                    name,
                    email,
                    password_hash,
                    access_level as "access_level: AccessLevelModel",
                    bio,
                    avatar_url,
                    nickname,
                    years_of_experience,
                    created_at,
                    updated_at
            "#,
            uuid,
            user.name,
            user.email,
            password_hash,
            access_level as _
        )
        .fetch_one(self.pool.as_ref())
        .await;

        match result {
            Ok(model) => Ok(model.into()),
            Err(SqlxError::Database(db_err)) => {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if pg_err.code() == "23505" && pg_err.constraint() == Some("users_name_key") {
                        return Err(Error::Repository(RepositoryError::UsernameAlreadyTaken(
                            user.name.clone(),
                        )));
                    }
                    if pg_err.code() == "23505" && pg_err.constraint() == Some("users_email_key") {
                        return Err(Error::Repository(RepositoryError::EmailAlreadyTaken(
                            user.email.clone(),
                        )));
                    }
                }

                Err(Error::Repository(RepositoryError::DatabaseError(
                    SqlxError::Database(db_err),
                )))
            }

            Err(e) => Err(Error::Repository(RepositoryError::DatabaseError(e))),
        }
    }

    async fn update(&self, data: &UpdateUserCommand) -> Result<()> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");
        let mut separated = false;

        let mut add_separator = |builder: &mut QueryBuilder<Postgres>| {
            if separated {
                builder.push(", ");
            }
            separated = true;
        };

        if let Update::Change(name) = &data.name {
            add_separator(&mut query_builder);
            query_builder.push("name = ").push_bind(name);
        }
        if let Update::Change(email) = &data.email {
            add_separator(&mut query_builder);
            query_builder.push("email = ").push_bind(email);
        }
        if let Update::Change(password) = &data.password {
            add_separator(&mut query_builder);
            // todo: refact generate_hash to avoid cloning the String
            let hashed_password = generate_hash(password.clone()).await?;
            query_builder
                .push("password_hash = ")
                .push_bind(hashed_password);
        }
        if let Update::Change(bio) = &data.bio {
            add_separator(&mut query_builder);
            query_builder.push("bio = ").push_bind(bio);
        }
        if let Update::Change(avatar_url) = &data.avatar_url {
            add_separator(&mut query_builder);
            query_builder.push("avatar_url = ").push_bind(avatar_url);
        }
        if let Update::Change(nickname) = &data.nickname {
            add_separator(&mut query_builder);
            query_builder.push("nickname = ").push_bind(nickname);
        }
        if let Update::Change(years) = data.years_of_experience {
            add_separator(&mut query_builder);
            query_builder
                .push("years_of_experience = ")
                .push_bind(years.map(|value| value as i32));
        }

        if !separated {
            return Ok(());
        }

        query_builder.push(" WHERE id = ").push_bind(data.id);

        let result = query_builder
            .build()
            .execute(&*self.pool)
            .await
            .map_err(|e| Error::Repository(RepositoryError::DatabaseError(e)))?;

        if result.rows_affected() == 0 {
            return Err(Error::Repository(RepositoryError::UserNotFound));
        }

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<User>> {
        todo!()
    }

    async fn find_by_username(&self, name: &str) -> Result<User> {
        let result = sqlx::query_as!(
            UserModel,
            r#"
                SELECT
                    id,
                    name,
                    email,
                    password_hash,
                    access_level as "access_level: AccessLevelModel",
                    bio,
                    avatar_url,
                    nickname,
                    years_of_experience,
                    created_at,
                    updated_at
                FROM users WHERE name = $1
            "#,
            name
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| Error::Repository(RepositoryError::DatabaseError(e)))?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(RepositoryError::UserNotFound.into()),
        }
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        let result = sqlx::query_as!(
            UserModel,
            r#"
                SELECT
                    id,
                    name,
                    email,
                    password_hash,
                    access_level as "access_level: AccessLevelModel",
                    bio,
                    avatar_url,
                    nickname,
                    years_of_experience,
                    created_at,
                    updated_at
                FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| Error::Repository(RepositoryError::DatabaseError(e)))?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(RepositoryError::UserNotFound.into()),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
        let result = sqlx::query_as!(
            UserModel,
            r#"
                SELECT
                    id,
                    name,
                    email,
                    password_hash,
                    access_level as "access_level: AccessLevelModel",
                    bio,
                    avatar_url,
                    nickname,
                    years_of_experience,
                    created_at,
                    updated_at
                FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| Error::Repository(RepositoryError::DatabaseError(e)))?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(RepositoryError::UserNotFound.into()),
        }
    }

    async fn delete(&self, user_id: &Uuid) -> Result<User> {
        let result = sqlx::query_as!(
            UserModel,
            r#"
                DELETE FROM users WHERE id = $1 RETURNING
                    id,
                    name,
                    email,
                    password_hash,
                    access_level as "access_level: AccessLevelModel",
                    bio,
                    avatar_url,
                    nickname,
                    years_of_experience,
                    created_at,
                    updated_at
            "#,
            user_id,
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| Error::Repository(RepositoryError::DatabaseError(e)))?;

        match result {
            Some(user) => Ok(user.into()),
            None => Err(RepositoryError::UserNotFound.into()),
        }
    }
}
