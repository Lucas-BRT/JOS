use crate::{Result, domain::user::role::Role};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub role: Role,
}

impl Claims {
    pub fn new(sub: Uuid, exp: usize, iat: usize, role: Role) -> Self {
        Self {
            sub,
            exp,
            iat,
            role,
        }
    }
}

#[async_trait::async_trait]
pub trait TokenProvider: Send + Sync {
    async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String>;
    async fn decode_token(&self, token: &str) -> Result<Claims>;
}
