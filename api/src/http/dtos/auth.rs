use axum::{
    Json,
    response::{IntoResponse, Response},
};
use domain::entities::{CreateUserCommand, LoginUserCommand, User};
use serde::{Deserialize, Serialize};
use shared::prelude::Date;
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
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub joined_at: Date,
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
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}

// IntoResponse implementations
impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for LogoutResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for RefreshTokenResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
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

impl From<LoginRequest> for LoginUserCommand {
    fn from(req: LoginRequest) -> Self {
        LoginUserCommand {
            email: req.email,
            password: req.password,
        }
    }
}

impl From<RegisterRequest> for CreateUserCommand {
    fn from(req: RegisterRequest) -> Self {
        CreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}
