use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result, domain::user::role::Role};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub role: Role,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    expiration_duration: Duration,
}

impl JwtService {
    pub fn new(secret: String, expiration_duration: Duration) -> Self {
        Self { secret, expiration_duration }
    }

    pub fn generate_token(&self, user_id: Uuid, user_role: Role) -> Result<String> {
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

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_e| Error::InternalServerError)?; // map to domain/app error later if needed

        Ok(token_data.claims)
    }
}

// Abstraction to access JwtService from generic state
pub trait ProvidesJwtService {
    fn jwt_service(&self) -> &JwtService;
}


