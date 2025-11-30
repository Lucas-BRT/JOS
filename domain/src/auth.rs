use crate::entities::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use shared::Result;
use std::{ops::Add, time::Duration};
use uuid::Uuid;

pub const DEFAULT_MIN_DELAY_MILIS: u64 = 30;
pub const DEFAULT_MAX_DELAY_MILIS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenCommand {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct ChangePasswordCommand {
    pub current_password: String,
    pub new_password: String,
}

#[async_trait::async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn login(&self, command: LoginCommand) -> Result<LoginResponse>;
    async fn register(&self, command: RegisterCommand) -> Result<LoginResponse>;
    async fn refresh_token(&self, command: RefreshTokenCommand) -> Result<RefreshResponse>;
    async fn logout(&self, command: LogoutCommand) -> Result<()>;
    async fn validate_token(&self, token: &str) -> Result<Claims>;
    async fn change_password(&self, user_id: Uuid, command: ChangePasswordCommand) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct LogoutCommand {
    pub user_id: Uuid,
}

#[async_trait::async_trait]
pub trait PasswordProvider: Send + Sync {
    async fn generate_hash(&self, password: String) -> Result<String>;
    async fn verify_hash(&self, password: String, hash: String) -> Result<bool>;
    async fn validate_password(&self, password: &str) -> Result<()>;
}

#[async_trait::async_trait]
pub trait TokenProvider: Send + Sync {
    async fn generate_token(&self, user_id: Uuid) -> Result<String>;
    async fn decode_token(&self, token: &str) -> Result<Claims>;
}

impl Claims {
    pub fn new(user_id: Uuid, token_expiration_duration: Duration) -> Self {
        let now = Utc::now();
        let exp = now.add(token_expiration_duration).timestamp();
        let iat = now.timestamp();

        Self {
            sub: user_id,
            exp,
            iat,
        }
    }
}
