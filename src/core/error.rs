use crate::{
    application::error::ApplicationError, infrastructure::repositories::error::RepositoryError,
    setup::SetupError,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use validator::ValidationErrors;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed during application startup: {0}")]
    Setup(SetupError),
    #[error("Application error: {0}")]
    Application(ApplicationError),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
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

            },
            Error::InternalServerError => {
                tracing::error!("Internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}
