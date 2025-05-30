use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub struct ValidationErrorsWrapper(pub ValidationErrors);

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub fn validation_errors_to_response(errors: &ValidationErrors) -> ErrorResponse {
    let details = errors
        .field_errors()
        .iter()
        .flat_map(|(field, errs)| {
            errs.iter().map(move |err| FieldError {
                field: field.to_string(),
                code: err.code.to_string(),
                message: err.message.as_ref().map(|m| m.to_string()),
            })
        })
        .collect();

    ErrorResponse {
        error: "Validation failed".into(),
        details: Some(details),
    }
}

impl From<ValidationErrors> for ValidationErrorsWrapper {
    fn from(e: ValidationErrors) -> Self {
        Self(e)
    }
}

impl IntoResponse for ValidationErrorsWrapper {
    fn into_response(self) -> Response {
        let error_response = validation_errors_to_response(&self.0);
        (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self)).into_response()
    }
}

impl IntoResponse for FieldError {
    fn into_response(self) -> Response {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self)).into_response()
    }
}
