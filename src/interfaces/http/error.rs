use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Field already provided: {field}")]
    DuplicatedFieldError { field: String },
    #[error("Invalid image format: {format}")]
    InvalidImageFormat { format: String },
    #[error("Image size too big: {limit}")]
    InvalidImageSize { limit: u64 },
    #[error("Missing field: {field}")]
    MissingField { field: String },
    #[error("Invalid player slots: {field}")]
    InvalidPlayerSlots { field: String },
    #[error("Invalid boolean value: {value}")]
    InvalidBooleanValue { value: String },
    #[error("Invalid game ID: {value}")]
    InvalidGameId { value: String },
    #[error("Invalid GM ID: {value}")]
    InvalidGmId { value: String },
    #[error("Bad request: {reason}")]
    BadRequest { reason: String },
    #[error("password and password_confirmation mismatch")]
    PasswordMismatch,
    #[error("{0}")]
    Other(#[from] validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidImageSize { limit } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "invalid-image-size",
                    "reason": format!("maxium allowed image size: {}", limit)
                })),
            )
                .into_response(),
            Self::DuplicatedFieldError { field } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "duplicated-field",
                    "reason": field
                })),
            )
                .into_response(),
            Self::InvalidImageFormat { format } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "invalid-image-format",
                        "image": format
                })),
            )
                .into_response(),
            Self::MissingField { field } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "missing-field",
                    "field": field
                })),
            )
                .into_response(),
            Self::PasswordMismatch => {
                tracing::error!("Password confirmation mismatch");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "password_confirmation": ["Passwords do not match"]
                    })),
                )
                    .into_response()
            }
            Self::Other(errors) => {
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
            Self::BadRequest { reason } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": reason
                })),
            )
                .into_response(),
            Self::InvalidGmId { value } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": value
                })),
            )
                .into_response(),
            Self::InvalidPlayerSlots { field } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": field
                })),
            )
                .into_response(),
            Self::InvalidBooleanValue { value } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": value
                })),
            )
                .into_response(),
            Self::InvalidGameId { value } => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": value
                })),
            )
                .into_response(),
        }
    }
}
