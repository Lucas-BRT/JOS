use axum::{
    Json,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain::entities::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 6))]
    pub current_password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
    #[validate(length(min = 6))]
    pub confirm_password: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct DeleteAccountRequest {
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub joined_at: DateTime<Utc>,
}

impl From<User> for UpdateProfileResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            joined_at: value.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteAccountResponse {
    pub message: String,
}

// IntoResponse implementations
impl IntoResponse for UpdateProfileResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for ChangePasswordResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for DeleteAccountResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
