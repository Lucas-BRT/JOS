use crate::{
    application::error::ApplicationError,
    domain::error::DomainError,
    infrastructure::persistance::{error::RepositoryError, postgres::error::translate_db_error},
    interfaces::http::error::ErrorResponse,
};
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed during application startup: {0}")]
    ApplicationSetup(String),
    #[error("Domain error: {0}")]
    Domain(DomainError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Repository error: {0}")]
    Repository(RepositoryError),
    #[error("Validation error: ")]
    Validation(ErrorResponse),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ApplicationSetup(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": msg })),
            )
                .into_response(),
            AppError::Database(err) => translate_db_error(&err).into_response(),
            AppError::Domain(err) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            AppError::Application(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            AppError::Repository(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            AppError::Validation(err) => err.into_response(),
        }
    }
}
