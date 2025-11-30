use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::session_checkin::SessionCheckinData;
use domain::entities::*;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/{session_id}",
    security(("auth" = [])),
    summary = "Update a session",
    tag = "session",
)]
#[axum::debug_handler]
pub async fn update_session(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<UpdateSessionRequest>,
) -> Result<Json<UpdateSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let gm_id = claims.get_user_id();

    let updated_session = app_state
        .session_service
        .update_session_with_validation(
            gm_id,
            session_id,
            payload.title,
            payload.description,
            payload.scheduled_for.flatten(),
            payload.status.map(SessionStatus::from),
        )
        .await?;

    Ok(Json(updated_session.into()))
}

#[utoipa::path(
    delete,
    path = "/{session_id}",
    security(("auth" = [])),
    tag = "session",
    summary = "Delete a session"
)]
#[axum::debug_handler]
pub async fn delete_session(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<DeleteSessionResponse>> {
    let gm_id = claims.get_user_id();

    app_state
        .session_service
        .delete_session_with_validation(gm_id, session_id)
        .await?;

    Ok(Json(DeleteSessionResponse {
        message: format!("Session {} deleted successfully", session_id),
    }))
}

#[utoipa::path(
    post,
    path = "/{session_id}/start",
    security(("auth" = [])),
    tag = "session",
    summary = "Start a session (GM only)"
)]
pub async fn start_session(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<()>> {
    let gm_id = claims.get_user_id();

    app_state
        .session_service
        .start_session(gm_id, session_id)
        .await?;

    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/{session_id}/finalize",
    security(("auth" = [])),
    tag = "session",
    summary = "Finalize a session with check-ins (GM only)"
)]
pub async fn finalize_session_with_checkins(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<FinalizeSessionRequest>,
) -> Result<Json<SessionFinalizationResponse>> {
    let gm_id = claims.get_user_id();

    app_state
        .session_service
        .finalize_session_with_checkins(
            gm_id,
            session_id,
            payload
                .checkins
                .into_iter()
                .map(SessionCheckinData::from)
                .collect(),
        )
        .await?;

    Ok(Json(SessionFinalizationResponse {}))
}

pub fn session_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(finalize_session_with_checkins))
                .routes(routes!(start_session))
                .routes(routes!(update_session))
                .routes(routes!(delete_session)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
