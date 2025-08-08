use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::{
    Error, Result,
    domain::jwt::{Claims, JwtRepository},
};

pub struct JwtRepositoryImpl {
    secret: String,
    expiration_duration: Duration,
}

impl JwtRepositoryImpl {
    pub fn new(secret: String, expiration_duration: Duration) -> Self {
        Self { secret, expiration_duration }
    }
}

#[async_trait]
impl JwtRepository for JwtRepositoryImpl {
    async fn generate_token(&self, user_id: uuid::Uuid, user_role: crate::domain::user::role::Role) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(self.expiration_duration)
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            role: user_role,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("failed to generate JWT token {}", e);
            Error::InternalServerError
        })
    }

    async fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_e| Error::InternalServerError)?;

        Ok(token_data.claims)
    }
}
