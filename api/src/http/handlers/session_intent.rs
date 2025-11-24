use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::Json;
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::CreateSessionIntentCommand;
use infrastructure::state::AppState;
use shared::error::DomainError;
use shared::{Error, Result};
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
    let user = claims.get_user_id();

    let user = app_state.user_service.find_by_id(&user).await?;
    let table = match app_state
        .table_service
        .find_by_session_id(&session_id)
        .await?
    {
        Some(table) => table,
        None => {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "Can only create session intent in a table that already present"
                    .to_string(),
            }));
        }
    };
    let users = app_state
        .table_member_service
        .find_by_table_id(&table.id)
        .await?;

    if !users.iter().any(|u| u.user_id == user.id) {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "User must be a member of the table to create a session intent".to_string(),
        }));
    }

    app_state
        .session_service
        .submit_session_intent(CreateSessionIntentCommand {
            player_id: user.id,
            session_id,
            status: payload.intent.into(),
        })
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
