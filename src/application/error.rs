use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplicationError {
    #[error("invalid credentials")]
    InvalidCredentials,
}

impl From<ApplicationError> for Error {
    fn from(err: ApplicationError) -> Self {
        Error::Application(err)
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        match self {
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
        }
    }
}
