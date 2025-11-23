use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::Json;
use axum::extract::*;
use axum::middleware::from_fn_with_state;
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
    todo!()
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
    todo!()
}

#[utoipa::path(
    get,
    path = "/users/{user_id}/intents",
    tag = "session-intent",
    security(("auth" = [])),
    summary = "Get intents created by a specific user"
)]
#[axum::debug_handler]
pub async fn get_user_intents(
    claims: ClaimsExtractor,
    Path(user_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionIntentResponse>>> {
    todo!()
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
) -> Result<Json<SessionIntentResponse>> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/intents/{intent_id}",
    tag = "session-intent",
    security(("auth" = [])),
    summary = "Delete/cancel a session intent"
)]
#[axum::debug_handler]
pub async fn delete_session_intent(
    claims: ClaimsExtractor,
    Path(intent_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DeleteSessionIntentResponse>> {
    todo!()
}

pub fn session_intent_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(create_session_intent))
                .routes(routes!(get_session_intents))
                .routes(routes!(update_session_intent))
                .routes(routes!(delete_session_intent)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
