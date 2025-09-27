use crate::{Result, domain::auth::Claims};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/sessions",
    tag = "sessions",
    params(
        ("table_id" = Option<Uuid>, Query, description = "Filter by table ID"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of sessions", body = SessionListResponse)
    )
)]
pub async fn get_sessions(
    State(app_state): State<Arc<AppState>>,
    Query(filters): Query<SessionFilters>,
    _: Claims,
) -> Result<Json<SessionListResponse>> {
    let sessions = app_state.session_service.get_sessions(&filters).await?;
    Ok(Json(sessions))
}

#[utoipa::path(
    post,
    path = "/v1/sessions",
    tag = "sessions",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateSessionDto,
    responses(
        (status = 201, description = "Session created successfully", body = SessionResponse),
        (status = 400, description = "Bad request", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn create_session(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionDto>,
    claims: Claims,
) -> Result<Json<SessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    let session = app_state
        .session_service
        .create_session(&claims.user_id, &payload.into())
        .await?;
    Ok(Json(session.into()))
}

#[utoipa::path(
    get,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session details", body = SessionResponse),
        (status = 404, description = "Session not found", body = serde_json::Value)
    )
)]
pub async fn get_session_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<SessionResponse>> {
    let session = app_state
        .session_service
        .get_session_by_id(&session_id)
        .await?;
    Ok(Json(session.into()))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = UpdateSessionDto,
    responses(
        (status = 200, description = "Session updated successfully", body = SessionResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn update_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<UpdateSessionDto>,
    claims: Claims,
) -> Result<Json<SessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    let session = app_state
        .session_service
        .update_session(&session_id, &claims.user_id, &payload.into())
        .await?;
    Ok(Json(session.into()))
}

#[utoipa::path(
    delete,
    path = "/v1/sessions/{id}",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Session deleted successfully"),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn delete_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<serde_json::Value>> {
    app_state
        .session_service
        .delete_session(&session_id, &claims.user_id)
        .await?;
    Ok(Json(
        serde_json::json!({"message": "Session deleted successfully"}),
    ))
}

#[utoipa::path(
    post,
    path = "/v1/sessions/{id}/join",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = JoinSessionDto,
    responses(
        (status = 200, description = "Joined session successfully", body = JoinSessionResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn join_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<JoinSessionDto>,
    claims: Claims,
) -> Result<Json<JoinSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    app_state
        .session_service
        .join_session(&session_id, &claims.user_id, &payload.character_name)
        .await?;
    Ok(Json(JoinSessionResponse {
        message: "Joined session successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/sessions/{id}/leave",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Left session successfully", body = LeaveSessionResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn leave_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<LeaveSessionResponse>> {
    app_state
        .session_service
        .leave_session(&session_id, &claims.user_id)
        .await?;
    Ok(Json(LeaveSessionResponse {
        message: "Left session successfully".to_string(),
    }))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}/confirm",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Session confirmed successfully", body = ConfirmSessionResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn confirm_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<ConfirmSessionResponse>> {
    app_state
        .session_service
        .confirm_session(&session_id, &claims.user_id)
        .await?;
    Ok(Json(ConfirmSessionResponse {
        message: "Session confirmed successfully".to_string(),
    }))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}/decline",
    tag = "sessions",
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Session declined successfully", body = DeclineSessionResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
pub async fn decline_session(
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<DeclineSessionResponse>> {
    app_state
        .session_service
        .decline_session(&session_id, &claims.user_id)
        .await?;
    Ok(Json(DeclineSessionResponse {
        message: "Session declined successfully".to_string(),
    }))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_sessions))
        .route("/", post(create_session))
        .route("/{id}", get(get_session_by_id))
        .route("/{id}", put(update_session))
        .route("/{id}", delete(delete_session))
        .route("/{id}/join", post(join_session))
        .route("/{id}/leave", delete(leave_session))
        .route("/{id}/confirm", put(confirm_session))
        .route("/{id}/decline", put(decline_session))
        .with_state(state.clone())
}
