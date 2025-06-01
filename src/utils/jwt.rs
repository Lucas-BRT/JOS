use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const JWT_EXPIRATION_TIME: Duration = Duration::hours(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
    pub email: String,
}

impl Claims {
    pub fn new(sub: String, role: String, email: String) -> Self {
        let exp = (Utc::now() + JWT_EXPIRATION_TIME).timestamp() as usize;
        Claims {
            sub,
            exp,
            role,
            email,
        }
    }
}

static JWT_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET must be defined"));

pub static ENCODING_KEY: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_secret(JWT_SECRET.as_bytes()));

pub static DECODING_KEY: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_secret(JWT_SECRET.as_bytes()));

pub const JWT_ALGORITHM: Algorithm = Algorithm::RS256;
