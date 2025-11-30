use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::Json;
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::IntentStatus;
use infrastructure::state::AppState;
use shared::Result;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/{session_id}/intents",
    tag = "session-intent",
    security(("auth" = [])),
    summary = "Create a session intent"
)]
#[axum::debug_handler]
pub async fn create_session_intent(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionIntentRequest>,
) -> Result<Json<CreateSessionIntentResponse>> {
    let user_id = claims.get_user_id();

    app_state
        .session_intent_service
        .create_with_validation(user_id, session_id, payload.intent.into())
        .await?;

    Ok(Json(CreateSessionIntentResponse {}))
}

#[utoipa::path(
    get,
    path = "/{session_id}/intents",
    tag = "session-intent",
    security(("auth" = [])),
    summary = "Get intents for a specific session"
)]
#[axum::debug_handler]
pub async fn get_session_intents(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionIntentResponse>>> {
    let user_id = claims.get_user_id();

    let intents = app_state
        .session_intent_service
        .get_for_session_with_validation(user_id, session_id)
        .await?;

    Ok(Json(
        intents
            .into_iter()
            .map(SessionIntentResponse::from)
            .collect(),
    ))
}

#[utoipa::path(
    put,
    path = "/intents/{intent_id}",
    tag = "session-intent",
    security(("auth" = [])),
    summary = "Update an existing session intent"
)]
#[axum::debug_handler]
pub async fn update_session_intent(
    claims: ClaimsExtractor,
    Path(intent_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateSessionIntentRequest>,
) -> Result<Json<()>> {
    let user_id = claims.get_user_id();

    app_state
        .session_intent_service
        .update_with_validation(user_id, intent_id, payload.status.map(IntentStatus::from))
        .await?;

    Ok(Json(()))
}

pub fn session_intent_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(create_session_intent))
                .routes(routes!(get_session_intents))
                .routes(routes!(update_session_intent)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
