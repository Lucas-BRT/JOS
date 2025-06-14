use crate::Error;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("username already taken: {0}")]
    UsernameAlreadyTaken(String),
    #[error("email already taken: {0}")]
    EmailAlreadyTaken(String),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user not found")]
    UserNotFound,
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        Error::Repository(err)
    }
}

impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        match self {
            Self::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Internal Server Error"
                    })),
                )
                    .into_response()
            }
            Self::UsernameAlreadyTaken(username) => {
                tracing::error!("Username already taken: {}", username);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Username already taken",
                        "value": username
                    })),
                )
                    .into_response()
            }
            Self::EmailAlreadyTaken(email) => {
                tracing::error!("Email already taken: {}", email);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Email already taken",
                        "value": email
                    })),
                )
                    .into_response()
            }
            Self::UserNotFound => {
                tracing::warn!("User not found");

                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "not found"
                    })),
                )
                    .into_response()
            }
        }
    }
}
