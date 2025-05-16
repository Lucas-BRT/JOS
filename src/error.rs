use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    ApplicationSetup(String),
    #[error("Validation error: {0}")]
    Validation(ValidationError),
    #[error("Database error: {0}")]
    Database(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ApplicationSetup(msg) => {
                let body = Json(json!({ "error": msg }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            Error::Validation(err) => err.into_response(), // delega pro ValidationError
            Error::Database(err) => {
                let body = Json(json!({ "error": err.to_string() }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("User validation failed: {0}")]
    User(#[from] UserValidationError),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            ValidationError::User(e) => e.into_response(), // delega pro UserValidationError
        }
    }
}

#[derive(Error, Debug)]
pub enum UserValidationError {
    #[error("failed to parse username: {0}")]
    Username(String),

    #[error("failed to parse display name: {0}")]
    DisplayName(String),

    #[error("failed to parse email: {0}")]
    Email(String),

    #[error("failed to parse password: {0}")]
    Password(String),

    #[error("failed to parse user role: {0}")]
    UserRole(String),
}

impl IntoResponse for UserValidationError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        let body = Json(json!({ "error": msg }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
