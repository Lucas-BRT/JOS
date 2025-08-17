use crate::{
    application::error::ApplicationError, domain::error::DomainError,
    infrastructure::repositories::error::RepositoryError, setup::SetupError,
};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    Setup(SetupError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Domain error: {0}")]
    Domain(DomainError),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
    #[error("Repository error: {0}")]
    Repository(RepositoryError),
    #[error("Internal server error")]
    InternalServerError,
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
            Self::InvalidInput(message) => {
                tracing::warn!("invalid input: {}", message);

                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for DomainError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Password(err) => {
                tracing::error!("Password error: {}", err);
                (StatusCode::BAD_REQUEST, "Password error").into_response()
            }
            Self::User(err) => {
                tracing::error!("User error: {}", err);
                (StatusCode::BAD_REQUEST, "User error").into_response()
            }
            Self::Table(err) => {
                tracing::error!("Table error: {}", err);
                (StatusCode::BAD_REQUEST, "Table error").into_response()
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Setup(err) => {
                tracing::error!("Application setup error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Application(err) => err.into_response(),
            Error::Domain(err) => err.into_response(),
            Error::Repository(err) => {
                tracing::error!("Repository error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Validation(err) => {
                let errors = err
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
            Error::InternalServerError => {
                tracing::error!("Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}
