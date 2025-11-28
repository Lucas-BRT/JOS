use crate::http::dtos::session_management::{
    CreateSessionCheckinRequest, CreateSessionCheckinResponse, SessionCheckinResponse,
    UpdateSessionCheckinRequest,
};
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::{Json, extract::Path, extract::State, middleware::from_fn_with_state};
use infrastructure::state::AppState;
use shared::Result;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/{session_intent_id}/checkin",
    tag = "session-checkin",
    security(("auth" = [])),
    summary = "Create a session checkin"
)]
#[axum::debug_handler]
async fn create_checkin(
    State(app_state): State<Arc<AppState>>,
    Path(session_intent_id): Path<Uuid>,
    claims: ClaimsExtractor,
    Json(payload): Json<CreateSessionCheckinRequest>,
) -> Result<Json<CreateSessionCheckinResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(shared::Error::Validation(validation_error));
    }

    todo!()
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

    todo!()
}

pub fn session_checkin_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(create_checkin))
                .routes(routes!(update_checkin)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
