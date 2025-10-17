use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/sessions",
    tag = "sessions",
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = Vec<SessionListItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_sessions(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionListItem>>> {
    // TODO: Implement session listing logic
    // For now, return empty list
    Ok(Json(vec![]))
}

#[utoipa::path(
    post,
    path = "/v1/sessions",
    tag = "sessions",
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
    State(_app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionDetails>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    // TODO: Implement session creation logic
    // For now, return a placeholder response
    Ok(Json(SessionDetails {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: "Awaiting Confirmations".to_string(),
        scheduled_at: payload.scheduled_at,
        accepting_proposals: true,
        updated_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        date: payload.scheduled_at.format("%Y-%m-%d").to_string(),
        time: payload.scheduled_at.format("%H:%M").to_string(),
        max_players: payload.max_players,
        master_id: claims.0.sub,
        table_id: payload.table_id,
        players: vec![],
    }))
}

#[utoipa::path(
    get,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session details retrieved", body = SessionDetails),
        (status = 404, description = "Session not found", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_session_details(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<SessionDetails>> {
    // TODO: Implement session details retrieval
    // For now, return a placeholder response
    Ok(Json(SessionDetails {
        id: session_id,
        title: "Placeholder Session".to_string(),
        description: "Placeholder description".to_string(),
        status: "Awaiting Confirmations".to_string(),
        scheduled_at: chrono::Utc::now(),
        accepting_proposals: true,
        updated_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        time: chrono::Utc::now().format("%H:%M").to_string(),
        max_players: 4,
        master_id: Uuid::new_v4(),
        table_id: Uuid::new_v4(),
        players: vec![],
    }))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    request_body = UpdateSessionRequest,
    responses(
        (status = 200, description = "Session updated successfully", body = SessionDetails),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to update this session", body = ErrorResponse),
        (status = 404, description = "Session not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_session(
    claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<UpdateSessionRequest>,
) -> Result<Json<SessionDetails>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    // TODO: Implement session update logic
    // For now, return a placeholder response
    Ok(Json(SessionDetails {
        id: session_id,
        title: payload.title.unwrap_or("Updated Session".to_string()),
        description: payload
            .description
            .unwrap_or("Updated description".to_string()),
        status: payload
            .status
            .unwrap_or("Awaiting Confirmations".to_string()),
        scheduled_at: payload.scheduled_at.unwrap_or(chrono::Utc::now()),
        accepting_proposals: true,
        updated_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        date: payload
            .scheduled_at
            .unwrap_or(chrono::Utc::now())
            .format("%Y-%m-%d")
            .to_string(),
        time: payload
            .scheduled_at
            .unwrap_or(chrono::Utc::now())
            .format("%H:%M")
            .to_string(),
        max_players: payload.max_players.unwrap_or(4),
        master_id: claims.0.sub,
        table_id: Uuid::new_v4(),
        players: vec![],
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/sessions/{id}",
    tag = "sessions",
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
            "/sessions",
            Router::new()
                .route("/", get(get_sessions))
                .route("/", post(create_session))
                .route("/{id}", get(get_session_details))
                .route("/{id}", put(update_session))
                .route("/{id}", delete(delete_session)),
        )
        .with_state(state)
}
