use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Field already provided: {0}")]
    DuplicatedFieldError(String),
    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),
    #[error("Image too big: {0}")]
    ImageTooBig(String),
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("Invalid player slots: {0}")]
    InvalidPlayerSlots(String),
    #[error("Invalid boolean value: {0}")]
    InvalidBooleanValue(String),
    #[error("Invalid game ID: {0}")]
    InvalidGameId(String),
    #[error("Invalid GM ID: {0}")]
    InvalidGmId(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("password and password_confirmation mismatch")]
    PasswordMismatch,
    #[error("{0}")]
    Other(#[from] validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::DuplicatedFieldError(field) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": field
                })),
            )
                .into_response(),
            Self::ImageTooBig(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "errors": {
                        "image": [msg]
                    }
                })),
            )
                .into_response(),
            Self::InvalidImageFormat(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "errors": {
                        "image": [msg]
                    }
                })),
            )
                .into_response(),
            Self::MissingField(field) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "errors": {
                        field: ["Field is required"]
                    }
                })),
            )
                .into_response(),
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
            ValidationError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": message
                })),
            )
                .into_response(),
            ValidationError::InvalidGmId(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": message
                })),
            )
                .into_response(),
            ValidationError::InvalidPlayerSlots(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": message
                })),
            )
                .into_response(),
            ValidationError::InvalidBooleanValue(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": message
                })),
            )
                .into_response(),
            ValidationError::InvalidGameId(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": message
                })),
            )
                .into_response(),
        }
    }
}
