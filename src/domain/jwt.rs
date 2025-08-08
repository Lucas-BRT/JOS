use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Result, domain::user::role::Role};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub role: Role,
}

#[async_trait]
pub trait JwtRepository: Send + Sync {
    async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String>;
    async fn decode_token(&self, token: &str) -> Result<Claims>;
}
