use crate::domain::models::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MeResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        MeResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 4, max = 100))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordDto {
    #[validate(length(min = 8, max = 200))]
    pub current_password: String,
    #[validate(length(min = 8, max = 200))]
    pub new_password: String,
    #[validate(must_match(other = "new_password"))]
    pub confirm_new_password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateUserResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteUserResponse {
    pub message: String,
}