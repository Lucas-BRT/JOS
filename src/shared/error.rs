use crate::{
    adapters::outbound::postgres::RepositoryError, domain::error::DomainError,
    infrastructure::error::SetupError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Domain error: {0}")]
    Domain(crate::domain::error::DomainError),
    #[error("Application error: {0}")]
    Application(crate::application::error::ApplicationError),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Setup error: {0}")]
    Setup(SetupError),
    #[error("Persistence error: {0}")]
    Persistence(RepositoryError),
}

impl From<DomainError> for Error {
    fn from(err: DomainError) -> Self {
        Error::Domain(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Domain(_) => (StatusCode::BAD_REQUEST, "Domain error"),
            Error::Application(_) => (StatusCode::BAD_REQUEST, "Application error"),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::Setup(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Setup error"),
            Error::Persistence(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Persistence error"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string()
        }));

        (status, body).into_response()
    }
}
