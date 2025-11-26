use crate::http::dtos::session_management::{
    CreateSessionCheckinRequest, CreateSessionCheckinResponse, DeleteSessionCheckinResponse,
    SessionCheckinResponse, UpdateSessionCheckinRequest,
};
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::{Json, extract::Path, extract::State, middleware::from_fn_with_state};
use domain::entities::{
    CreateSessionCheckinCommand, DeleteSessionCheckinCommand, GetSessionCheckinCommand, Update,
    UpdateSessionCheckinCommand,
};
use infrastructure::state::AppState;
use shared::Result;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/",
    tag = "session-checkin",
    security(("auth" = [])),
    summary = "Create a session checkin"
)]
#[axum::debug_handler]
async fn create_checkin(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
    Json(payload): Json<CreateSessionCheckinRequest>,
) -> Result<Json<CreateSessionCheckinResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(shared::Error::Validation(validation_error));
    }

    let _user_id = claims.get_user_id();

    // TODO: Add business authorization checks:
    // - ensure the session intent exists
    // - ensure the user is allowed to create a checkin for that intent

    let command = CreateSessionCheckinCommand {
        session_intent_id: payload.session_intent_id,
        attendance: payload.attendance,
        notes: payload.notes,
    };

    let created = app_state.session_checkin_service.create(command).await?;

    Ok(Json(CreateSessionCheckinResponse { id: created.id }))
}

#[utoipa::path(
    get,
    path = "/{session_intent_id}/checkins",
    tag = "session-checkin",
    security(("auth" = [])),
    summary = "Get checkins for a session intent"
)]
#[axum::debug_handler]
async fn get_checkins_by_session_intent(
    State(app_state): State<Arc<AppState>>,
    Path(session_intent_id): Path<Uuid>,
) -> Result<Json<Vec<SessionCheckinResponse>>> {
    let command = GetSessionCheckinCommand {
        session_intent_id: Some(session_intent_id),
        ..Default::default()
    };

    let checkins = app_state.session_checkin_service.get(command).await?;

    Ok(Json(checkins.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    put,
    path = "/{checkin_id}",
    tag = "session-checkin",
    security(("auth" = [])),
    summary = "Update a session checkin"
)]
#[axum::debug_handler]
async fn update_checkin(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
    Path(checkin_id): Path<Uuid>,
    Json(payload): Json<UpdateSessionCheckinRequest>,
) -> Result<Json<SessionCheckinResponse>> {
    let _user_id = claims.get_user_id();

    let command = UpdateSessionCheckinCommand {
        id: checkin_id,
        session_intent_id: match payload.session_intent_id {
            Some(id) => Update::Change(id),
            None => Update::Keep,
        },
        attendance: match payload.attendance {
            Some(a) => Update::Change(a),
            None => Update::Keep,
        },
        notes: match payload.notes {
            Some(n) => Update::Change(n),
            None => Update::Keep,
        },
    };

    let updated = app_appstate_or_placeholder(&app_state)
        .session_checkin_service
        .update(command)
        .await?;

    Ok(Json(updated.into()))
}

#[utoipa::path(
    delete,
    path = "/{checkin_id}",
    tag = "session-checkin",
    security(("auth" = [])),
    summary = "Delete a session checkin"
)]
#[axum::debug_handler]
async fn delete_checkin(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
    Path(checkin_id): Path<Uuid>,
) -> Result<Json<DeleteSessionCheckinResponse>> {
    let _user_id = claims.get_user_id();

    let command = DeleteSessionCheckinCommand { id: checkin_id };

    app_state.session_checkin_service.delete(command).await?;

    Ok(Json(DeleteSessionCheckinResponse {
        message: format!("Checkin {} deleted successfully", checkin_id),
    }))
}
pub fn session_checkin_routes(state: Arc<AppState>) -> OpenApiRouter {
    let checkin_ops = OpenApiRouter::new()
        .routes(routes!(create_checkin))
        .routes(routes!(update_checkin))
        .routes(routes!(delete_checkin));

    let by_intent = OpenApiRouter::new().routes(routes!(get_checkins_by_session_intent));

    OpenApiRouter::new()
        .nest("/checkins", OpenApiRouter::new().merge(checkin_ops))
        .nest("/sessions", OpenApiRouter::new().merge(by_intent))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
