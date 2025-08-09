use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ValidationError {
    #[error("password and password_confirmation mismatch")]
    PasswordMismatch,
    #[error("{0}")]
    Other(#[from] validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            ValidationError::PasswordMismatch => {
                tracing::error!("Password confirmation mismatch");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "password_confirmation": ["Passwords do not match"]
                    })),
                )
                    .into_response()
            }
            ValidationError::Other(errors) => {
                let errors = errors
                    .errors()
                    .clone()
                    .into_keys()
                    .map(|key| key.to_string())
                    .collect::<Vec<String>>();

                tracing::error!("Validation error: {:?}", errors);

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "validation": errors
                    })),
                )
                    .into_response()
            }
        }
    }
}

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
