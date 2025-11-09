use chrono::Duration;
use domain::{auth::Claims, repositories::TokenProvider};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use shared::error::{Error, InfrastructureError};
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
    async fn generate_token(&self, user_id: Uuid) -> Result<String, Error> {
        let claims = Claims::new(user_id, self.expiration_duration);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|error| {
            Error::Infrastructure(InfrastructureError::FailedToEncodeJwt(error.to_string()))
        })
    }

    async fn decode_token(&self, token: &str) -> Result<Claims, Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|error| {
            Error::Infrastructure(InfrastructureError::FailedToDecodeJwt(error.to_string()))
        })?;

        Ok(token_data.claims)
    }
}
