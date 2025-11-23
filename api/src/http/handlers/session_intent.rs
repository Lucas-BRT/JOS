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

/// Handlers for session intents (players expressing intent to attend a session)
///
/// Endpoints provided (skeletons):
/// - POST   /{session_id}/intents          -> create a new session intent (player)
/// - GET    /{session_id}/intents          -> list intents for a session (GM / auth)
/// - GET    /users/{user_id}/intents       -> list intents by user
/// - PUT    /intents/{intent_id}           -> update an intent (e.g. change status)
/// - DELETE /intents/{intent_id}           -> delete/cancel an intent
///
/// Each handler body is left as `todo!()` to be implemented following the application's
/// patterns and services. Utoipa annotations are provided to generate OpenAPI docs.
#[utoipa::path(
    post,
    path = "/{session_id}/intents",
    tags = ["SessionIntent"],
    security(("auth" = [])),
    summary = "Create a session intent (player indicates they will attend)"
)]
#[axum::debug_handler]
pub async fn create_session_intent(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionIntentRequest>,
) -> Result<Json<CreateSessionIntentResponse>> {
    // validate payload if needed, then call application/service layer to create
    // e.g. app_state.session_intent_service.create(CreateSessionIntentCommand { ... })
    todo!()
}

#[utoipa::path(
    get,
    path = "/{session_id}/intents",
    tags = ["SessionIntent"],
    security(("auth" = [])),
    summary = "Get intents for a specific session"
)]
#[axum::debug_handler]
pub async fn get_session_intents(
    claims: ClaimsExtractor,
    Path(session_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionIntentResponse>>> {
    // Should use app_state.session_intent_service.find_by_session_id(&session_id)
    todo!()
}

#[utoipa::path(
    get,
    path = "/users/{user_id}/intents",
    tags = ["SessionIntent"],
    security(("auth" = [])),
    summary = "Get intents created by a specific user"
)]
#[axum::debug_handler]
pub async fn get_user_intents(
    claims: ClaimsExtractor,
    Path(user_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionIntentResponse>>> {
    // Should use app_state.session_intent_service.find_by_user_id(&user_id)
    todo!()
}

#[utoipa::path(
    put,
    path = "/intents/{intent_id}",
    tags = ["SessionIntent"],
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
    // Should map UpdateSessionIntentRequest -> UpdateSessionIntentCommand and call service
    todo!()
}

#[utoipa::path(
    delete,
    path = "/intents/{intent_id}",
    tags = ["SessionIntent"],
    security(("auth" = [])),
    summary = "Delete/cancel a session intent"
)]
#[axum::debug_handler]
pub async fn delete_session_intent(
    claims: ClaimsExtractor,
    Path(intent_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DeleteSessionIntentResponse>> {
    // Should call app_state.session_intent_service.delete(DeleteSessionIntentCommand { id: intent_id })
    todo!()
}

pub fn session_intent_routes(state: Arc<AppState>) -> OpenApiRouter {
    // All endpoints require authentication
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(create_session_intent))
                .routes(routes!(get_session_intents)),
        )
        .merge(
            OpenApiRouter::new()
                .nest(
                    "/users",
                    OpenApiRouter::new().routes(routes!(get_user_intents)),
                )
                .merge(OpenApiRouter::new().routes(routes!(update_session_intent)))
                .merge(OpenApiRouter::new().routes(routes!(delete_session_intent))),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

// ---------------------- Session Checkin handlers ----------------------
//
// Handlers for session checkins (attendance records tied to session intents)
//
// Endpoints (skeletons):
// - POST   /session_intents/{intent_id}/checkins    -> create checkin for an intent (GM or automated)
// - GET    /session_intents/{intent_id}/checkins    -> list checkins for an intent
// - PUT    /session_checkins/{id}                   -> update a checkin
// - DELETE /session_checkins/{id}                   -> delete a checkin
//
// Each handler body is left as `todo!()` and documented with utoipa.

#[utoipa::path(
    post,
    path = "/session_intents/{intent_id}/checkins",
    tags = ["SessionCheckin"],
    security(("auth" = [])),
    summary = "Create a session checkin (attendance) for an intent"
)]
#[axum::debug_handler]
pub async fn create_session_checkin(
    claims: ClaimsExtractor,
    Path(intent_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionCheckinRequest>,
) -> Result<Json<CreateSessionCheckinResponse>> {
    // Use app_state.session_checkin_service.create(CreateSessionCheckinCommand { ... })
    todo!()
}

#[utoipa::path(
    get,
    path = "/session_intents/{intent_id}/checkins",
    tags = ["SessionCheckin"],
    security(("auth" = [])),
    summary = "Get checkins for a given session intent"
)]
#[axum::debug_handler]
pub async fn get_session_checkins(
    claims: ClaimsExtractor,
    Path(intent_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionCheckinResponse>>> {
    // Use app_state.session_checkin_service.find_by_session_intent_id(&intent_id)
    todo!()
}

#[utoipa::path(
    put,
    path = "/session_checkins/{checkin_id}",
    tags = ["SessionCheckin"],
    security(("auth" = [])),
    summary = "Update a session checkin"
)]
#[axum::debug_handler]
pub async fn update_session_checkin(
    claims: ClaimsExtractor,
    Path(checkin_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateSessionCheckinRequest>,
) -> Result<Json<SessionCheckinResponse>> {
    // Map payload to UpdateSessionCheckinCommand and call service
    todo!()
}

#[utoipa::path(
    delete,
    path = "/session_checkins/{checkin_id}",
    tags = ["SessionCheckin"],
    security(("auth" = [])),
    summary = "Delete a session checkin"
)]
#[axum::debug_handler]
pub async fn delete_session_checkin(
    claims: ClaimsExtractor,
    Path(checkin_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DeleteSessionCheckinResponse>> {
    // Call app_state.session_checkin_service.delete(DeleteSessionCheckinCommand { id: checkin_id })
    todo!()
}

pub fn session_checkin_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/session_intents",
            OpenApiRouter::new()
                .routes(routes!(create_session_checkin))
                .routes(routes!(get_session_checkins)),
        )
        .merge(
            OpenApiRouter::new()
                .routes(routes!(update_session_checkin))
                .routes(routes!(delete_session_checkin)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
