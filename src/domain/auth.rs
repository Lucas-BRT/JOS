use crate::Result;
use crate::domain::entities::*;
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
    async fn authenticate(&self, command: &mut LoginUserCommand) -> Result<String>;
    async fn register(&self, command: &mut CreateUserCommand) -> Result<User>;
    async fn update_password(&self, command: &mut UpdatePasswordCommand) -> Result<()>;
}

#[async_trait]
pub trait PasswordProvider: Send + Sync {
    async fn generate_hash(&self, password: String) -> Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> Result<bool>;
    async fn validate_password(&self, password: &str) -> Result<()>;
}

#[async_trait]
pub trait TokenProvider: Send + Sync {
    async fn generate_token(&self, user_id: &Uuid) -> Result<String>;
    async fn decode_token(&self, token: &str) -> Result<Claims>;
}

impl Claims {
    pub fn new(user_id: Uuid, token_expiration_duration: Duration) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: now + token_expiration_duration,
            iat: now,
        }
    }
}
