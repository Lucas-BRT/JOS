use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseDto {
    token: String,
}

impl LoginResponseDto {
    pub fn new(token: String) -> Self {
        LoginResponseDto { token }
    }
}
