use crate::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(sub: Uuid, exp: usize, iat: usize) -> Self {
        Self { sub, exp, iat }
    }
}
