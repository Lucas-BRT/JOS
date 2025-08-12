use crate::Error;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("username already taken")]
    UsernameAlreadyTaken,
    #[error("email already taken")]
    EmailAlreadyTaken,
    #[error("game system name already taken: {0}")]
    GameSystemNameAlreadyTaken(String),
    #[error("user already has intent for this session")]
    UserSessionIntentAlreadyExists,
    #[error("foreign key violation: table {table}, field {field}")]
    ForeignKeyViolation { table: String, field: String },
    #[error("unknown constraint violation: {0}")]
    UnknownConstraint(String),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user not found")]
    UserNotFound,
    #[error("table not found")]
    TableNotFound,
}

impl From<RepositoryError> for Error {
    fn from(err: RepositoryError) -> Self {
        Error::Repository(err)
    }
}

impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        match self {
            Self::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Internal Server Error"
                    })),
                )
                    .into_response()
            }
            Self::UsernameAlreadyTaken => {
                tracing::error!("Username already taken");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Username already taken"
                    })),
                )
                    .into_response()
            }
            Self::EmailAlreadyTaken => {
                tracing::error!("Email already taken");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Email already taken"
                    })),
                )
                    .into_response()
            }
            Self::GameSystemNameAlreadyTaken(name) => {
                tracing::error!("Game system name already taken: {}", name);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Game system name already taken",
                        "value": name
                    })),
                )
                    .into_response()
            }
            Self::UserSessionIntentAlreadyExists => {
                tracing::error!("User already has intent for this session");
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "User already has intent for this session"
                    })),
                )
                    .into_response()
            }
            Self::ForeignKeyViolation { table, field } => {
                tracing::error!("Foreign key violation: table {}, field {}", table, field);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Foreign key violation",
                        "table": table,
                        "field": field
                    })),
                )
                    .into_response()
            }
            Self::UnknownConstraint(constraint) => {
                tracing::error!("Unknown constraint violation: {}", constraint);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Constraint violation",
                        "constraint": constraint
                    })),
                )
                    .into_response()
            }
            Self::UserNotFound => {
                tracing::warn!("User not found");

                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "not found"
                    })),
                )
                    .into_response()
            }
            Self::TableNotFound => {
                tracing::warn!("Table not found");
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": "Table not found"
                    })),
                )
                    .into_response()
            }
        }
    }
}


