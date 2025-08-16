use crate::domain::auth::{Claims, TokenProvider};
use crate::domain::user::Role;
use crate::{Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtTokenProvider {
    secret: String,
    expiration_duration: Duration,
}

impl JwtTokenProvider {
    pub fn new(secret: String, expiration_duration: Duration) -> Self {
        Self {
            secret,
            expiration_duration,
        }
    }
}

#[async_trait::async_trait]
impl TokenProvider for JwtTokenProvider {
    async fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(self.expiration_duration)
            .ok_or(Error::InternalServerError)?
            .timestamp();

        let claims = Claims::new(
            user_id,
            expiration as usize,
            Utc::now().timestamp() as usize,
            user_role,
        );

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| Error::InternalServerError)
    }

    async fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| Error::InternalServerError)?;

        Ok(token_data.claims)
    }
}
