use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        jwt::{JwtRepository},
        user::role::Role,
    },
    Result,
};

#[derive(Clone)]
pub struct JwtService {
    jwt_repository: Arc<dyn JwtRepository>,
}

impl JwtService {
    pub fn new(jwt_repository: Arc<dyn JwtRepository>) -> Self {
        Self { jwt_repository }
    }

    pub async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String> {
        self.jwt_repository.generate_token(user_id, user_role).await
    }

    pub async fn decode_token(&self, token: &str) -> Result<crate::domain::jwt::Claims> {
        self.jwt_repository.decode_token(token).await
    }
}

// Re-export Claims for public access
pub use crate::domain::jwt::Claims;


