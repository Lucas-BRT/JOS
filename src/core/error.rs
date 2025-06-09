use crate::{
    application::error::ApplicationError,
    infrastructure::persistance::postgres::repositories::error::RepositoryError,
    interfaces::http::error::ValidationError, setup::SetupError,
};
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    Setup(SetupError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Validation error: {0}")]
    Validation(ValidationError),
    #[error("Repository error: {0}")]
    Repository(RepositoryError),
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Setup(err) => {
                tracing::error!("Application setup error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Application(err) => err.into_response(),
            Error::Repository(err) => err.into_response(),
            Error::Validation(err) => err.into_response(),
            Error::InternalServerError => {
                tracing::error!("Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}
