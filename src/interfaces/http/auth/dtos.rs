use crate::domain::user::{dtos::{CreateUserCommand, LoginUserCommand}, entity::User};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupDto {
    #[validate(length(min = 4, max = 100))]
    pub name: String,
    #[validate(length(min = 4, max = 100))]
    pub nickname: String,
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

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserSignupResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            role: user.role.to_string(),
            created_at: user.created_at.to_string(),
        }
    }
}

impl From<SignupDto> for CreateUserCommand {
    fn from(dto: SignupDto) -> Self {
        CreateUserCommand {
            username: dto.name,
            display_name: dto.nickname,
            email: dto.email,
            password_hash: dto.password,
        }
    }
}

impl From<LoginDto> for LoginUserCommand {
    fn from(dto: LoginDto) -> Self {
        LoginUserCommand {
            email: dto.email,
            password: dto.password,
        }
    }
}

impl From<User> for UserSignupResponse {
    fn from(user: User) -> Self {
        UserSignupResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        }
    }
}

impl IntoResponse for UserSignupResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

