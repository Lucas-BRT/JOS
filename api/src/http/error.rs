use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use shared::error::Error;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub struct ApiError(pub Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0 {
            Error::Persistence(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            Error::Application(_) => (StatusCode::BAD_REQUEST, "Application error"),
            Error::Domain(_) => (StatusCode::BAD_REQUEST, "Domain error"),
            Error::Setup(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Setup error"),
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
