use crate::{Error, Result, domain::user::entity::AccessLevel};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // Subject (user id)
    pub exp: usize,
    pub iat: usize,
    pub access_level: AccessLevel,
}

impl Claims {
    pub fn create_jwt(
        user_id: Uuid,
        jwt_secret: &str,
        jwt_expiration_duration: Duration,
        user_access_level: AccessLevel,
    ) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(jwt_expiration_duration)
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            access_level: user_access_level,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("failed to generate JWT token {}", e);
            Error::InternalServerError
        })
    }
}
