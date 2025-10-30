use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::middleware::from_fn_with_state;
use axum::{extract::*, routing::*};
use domain::entities::{CreateSessionCommand, GetSessionCommand};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/tables/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = Vec<SessionListItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_sessions(
    Path(table_id): Path<Uuid>,
    _claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<GetSessionsResponse>>> {
    let sessions = app_state
        .session_service
        .get(GetSessionCommand {
            table_id: Some(table_id),
            ..Default::default()
        })
        .await?
        .iter()
        .map(GetSessionsResponse::from)
        .collect();

    Ok(Json(sessions))
}

#[utoipa::path(
    post,
    path = "/v1/tables/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    request_body = CreateSessionRequest,
    responses(
        (status = 201, description = "Session created successfully", body = SessionDetails),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to create session for this table", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_session(
    claims: ClaimsExtractor,
    Path(table_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let session = app_state
        .session_service
        .create(
            claims.0.sub,
            CreateSessionCommand {
                table_id,
                title: payload.title,
                description: payload.description,
                scheduled_for: payload.scheduled_for,
                status: payload.status.unwrap_or_default(),
            },
        )
        .await?;

    Ok(Json(CreateSessionResponse { id: session.id }))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    request_body = UpdateSessionRequest,
    responses(
        (status = 200, description = "Session updated successfully", body = UpdateSessionResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to update this session", body = ErrorResponse),
        (status = 404, description = "Session not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_session(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<UpdateSessionRequest>,
) -> Result<Json<UpdateSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    // TODO: Implement session update logic
    // For now, return a placeholder response
    Ok(Json(UpdateSessionResponse { id: session_id }))
}

#[utoipa::path(
    delete,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    security(("auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session deleted successfully", body = DeleteSessionResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to delete this session", body = ErrorResponse),
        (status = 404, description = "Session not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn delete_session(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<DeleteSessionResponse>> {
    // TODO: Implement session deletion logic
    Ok(Json(DeleteSessionResponse {
        message: format!("Session {} deleted successfully", session_id),
    }))
}

pub fn session_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/tables/{table_id}/sessions",
            Router::new()
                .route("/", post(create_session))
                .route("/", get(get_sessions)),
        )
        .nest(
            "/sessions",
            Router::new()
                .route("/", get(get_sessions))
                .route("/{id}", put(update_session))
                .route("/{id}", delete(delete_session)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
