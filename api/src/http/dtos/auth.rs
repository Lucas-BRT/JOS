use chrono::{DateTime, Utc};
use domain::entities::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub joined_at: DateTime<Utc>,
    pub email: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user: UserResponse,
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}

// Conversion implementations
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let username = user.username.clone();
        UserResponse {
            id: user.id,
            username: username.clone(),
            joined_at: user.created_at,
            email: user.email,
        }
    }
}
