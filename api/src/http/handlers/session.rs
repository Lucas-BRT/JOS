use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::*;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::{ApplicationError, DomainError, Error};
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

    let table = match app_state
        .table_service
        .find_by_session_id(&session_id)
        .await?
    {
        Some(table) => table,
        None => {
            return Err(Error::Domain(DomainError::EntityNotFound {
                entity_type: "session",
                entity_id: session_id.to_string(),
            }));
        }
    };

    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "User is not the GM of the table".to_string(),
        }));
    }

    let session = app_state.session_service.find_by_id(&session_id).await?;

    let status = Update::from(payload.status.map(SessionStatus::from));

    let command = UpdateSessionCommand {
        id: session.id,
        title: payload.title.into(),
        description: payload.description.into(),
        scheduled_for: payload.scheduled_for.into(),
        status,
    };

    let updated_session = app_state.session_service.update(command).await?;

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
    let table = match app_state
        .table_service
        .find_by_session_id(&session_id)
        .await?
    {
        Some(table) => table,
        None => {
            return Err(Error::Domain(DomainError::EntityNotFound {
                entity_type: "session",
                entity_id: session_id.to_string(),
            }));
        }
    };

    if claims.0.sub != table.gm_id {
        return Err(Error::Application(ApplicationError::InvalidCredentials));
    }

    app_state
        .session_service
        .delete(DeleteSessionCommand { id: session_id })
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

pub fn session_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(update_session))
                .routes(routes!(delete_session)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
