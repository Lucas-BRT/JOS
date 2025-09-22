use crate::domain::entities::{CreateUserCommand, LoginUserCommand, UpdateUserCommand, User};
use crate::shared::Date;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: Date,
    pub iat: Date,
}

#[async_trait]
pub trait Authenticator {
    async fn authenticate(&self, command: &LoginUserCommand) -> crate::Result<String>;
    async fn register(&self, command: &mut CreateUserCommand) -> crate::Result<User>;
    async fn update_password(&self, command: &mut UpdateUserCommand) -> crate::Result<()>;
}

#[async_trait]
pub trait PasswordProvider {
    async fn generate_hash(&self, password: String) -> crate::Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> crate::Result<bool>;
    async fn validate_password(&self, password: &str) -> crate::Result<()>;
}

#[async_trait]
pub trait TokenProvider {
    async fn generate_token(&self, user_id: &Uuid) -> crate::Result<String>;
    async fn decode_token(&self, token: &str) -> crate::Result<Claims>;
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: now + Duration::hours(24),
            iat: now,
        }
    }
}
