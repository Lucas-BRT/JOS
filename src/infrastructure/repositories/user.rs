use crate::Result;
use crate::domain::user::dtos::UpdateUserCommand;
use crate::domain::user::{
    dtos::CreateUserCommand, entity::User, user_repository::UserRepository as UserRepositoryTrait,
};
use crate::infrastructure::entities::{t_users::Model as UserModel, enums::ERoles};
use crate::infrastructure::repositories::error::RepositoryError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn map_role_to_entity(role: ERoles) -> crate::domain::user::role::Role {
        match role {
            ERoles::Admin => crate::domain::user::role::Role::Admin,
            ERoles::Moderator => crate::domain::user::role::Role::Moderator,
            ERoles::User => crate::domain::user::role::Role::User,
        }
    }

    pub fn map_role_to_db(role: &crate::domain::user::role::Role) -> ERoles {
        match role {
            crate::domain::user::role::Role::Admin => ERoles::Admin,
            crate::domain::user::role::Role::Moderator => ERoles::Moderator,
            crate::domain::user::role::Role::User => ERoles::User,
        }
    }

    pub fn map_model_to_entity(model: UserModel) -> User {
        User {
            id: model.id,
            name: model.name,
            email: model.email,
            password_hash: model.password_hash,
            role: Self::map_role_to_entity(model.role),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
        // Verificar se o username já existe
        let existing_user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE name = $1"
        )
        .bind(&user.name)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        if existing_user.is_some() {
            return Err(RepositoryError::UsernameAlreadyTaken(user.name.clone()).into());
        }

        // Verificar se o email já existe
        let existing_email = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE email = $1"
        )
        .bind(&user.email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        if existing_email.is_some() {
            return Err(RepositoryError::EmailAlreadyTaken(user.email.clone()).into());
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        let created_user = sqlx::query_as::<_, UserModel>(
            r#"
            INSERT INTO t_users (id, name, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(ERoles::User) // Default role
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(Self::map_model_to_entity(created_user))
    }

    async fn update(&self, data: &UpdateUserCommand) -> Result<()> {
        // Verificar se o usuário existe
        let existing_user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE id = $1"
        )
        .bind(data.id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        if existing_user.is_none() {
            return Err(RepositoryError::UserNotFound.into());
        }

        // Atualizar campos individualmente
        if let crate::domain::utils::update::Update::Change(name) = &data.name {
            // Verificar se o nome já existe para outro usuário
            let existing_name = sqlx::query_as::<_, UserModel>(
                "SELECT * FROM t_users WHERE name = $1 AND id != $2"
            )
            .bind(name)
            .bind(data.id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;

            if existing_name.is_some() {
                return Err(RepositoryError::UsernameAlreadyTaken(name.clone()).into());
            }

            sqlx::query("UPDATE t_users SET name = $1, updated_at = $2 WHERE id = $3")
                .bind(name)
                .bind(Utc::now())
                .bind(data.id)
                .execute(&*self.pool)
                .await
                .map_err(RepositoryError::DatabaseError)?;
        }

        if let crate::domain::utils::update::Update::Change(email) = &data.email {
            // Verificar se o email já existe para outro usuário
            let existing_email = sqlx::query_as::<_, UserModel>(
                "SELECT * FROM t_users WHERE email = $1 AND id != $2"
            )
            .bind(email)
            .bind(data.id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;

            if existing_email.is_some() {
                return Err(RepositoryError::EmailAlreadyTaken(email.clone()).into());
            }

            sqlx::query("UPDATE t_users SET email = $1, updated_at = $2 WHERE id = $3")
                .bind(email)
                .bind(Utc::now())
                .bind(data.id)
                .execute(&*self.pool)
                .await
                .map_err(RepositoryError::DatabaseError)?;
        }

        if let crate::domain::utils::update::Update::Change(password) = &data.password {
            sqlx::query("UPDATE t_users SET password_hash = $1, updated_at = $2 WHERE id = $3")
                .bind(password)
                .bind(Utc::now())
                .bind(data.id)
                .execute(&*self.pool)
                .await
                .map_err(RepositoryError::DatabaseError)?;
        }

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users ORDER BY created_at DESC"
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        Ok(users.into_iter().map(Self::map_model_to_entity).collect())
    }

    async fn find_by_username(&self, name: &str) -> Result<User> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(Self::map_model_to_entity)
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(Self::map_model_to_entity)
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM t_users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;

        user.map(Self::map_model_to_entity)
            .ok_or(RepositoryError::UserNotFound.into())
    }

    async fn delete(&self, user_id: &Uuid) -> Result<User> {
        // Primeiro buscar o usuário para retorná-lo
        let user = self.find_by_id(user_id).await?;

        // Deletar o usuário
        sqlx::query("DELETE FROM t_users WHERE id = $1")
            .bind(user_id)
            .execute(&*self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(user)
    }
}
