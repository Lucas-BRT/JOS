use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::error::DatabaseError;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplicationError {
    #[error("Data error: {0}")]
    DataError(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("user not found")]
    UserNotFound,
}

impl From<ApplicationError> for Error {
    fn from(err: ApplicationError) -> Self {
        Error::Application(err)
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataError(err) => {
                tracing::error!("database error: {}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": err})),
                )
                    .into_response()
            }
            Self::UserNotFound => {
                tracing::warn!("user not found");

                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": "User not found"})),
                )
                    .into_response()
            }
            Self::InvalidCredentials => {
                tracing::warn!("invalid credentials");

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "invalid credentials"
                    })),
                )
                    .into_response()
            }
            Self::NotFound(resource) => {
                tracing::error!("resource not found: {}", resource);

                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "resource": resource
                    })),
                )
                    .into_response()
            }
        }
    }
}
