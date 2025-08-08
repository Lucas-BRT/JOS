use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupDto {
    #[validate(length(min = 4, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 200))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 200))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserSignupResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
