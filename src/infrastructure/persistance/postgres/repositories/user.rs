use crate::Error;
use crate::Result;
use crate::domain::user::dtos::CreateUserCommand;
use crate::domain::user::dtos::UpdateUserCommand;
use crate::domain::user::entity::AccessLevel;
use crate::domain::user::entity::User;
use crate::domain::user::user_repository::UserRepository;
use crate::infrastructure::persistance::postgres::models::user::AccessLevelModel;
use crate::infrastructure::persistance::postgres::models::user::Model as UserModel;
use crate::infrastructure::persistance::postgres::repositories::error::RepositoryError;
use crate::utils::password::generate_hash;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::{Error as SqlxError, postgres::PgDatabaseError};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl<'a> PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<Uuid> {
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
            Ok(model) => Ok(model.id),
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

    async fn update(&self, user_id: &Uuid, data: &UpdateUserCommand) -> Result<()> {
        todo!()
    }
    async fn get_all(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn find_by_username(&self, name: &str) -> Result<Option<User>> {
        todo!()
    }
}
