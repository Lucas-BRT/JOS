use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
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
