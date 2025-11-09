use std::sync::Arc;

use crate::http::{
    dtos::*,
    error::HttpError,
    middleware::auth::{ClaimsExtractor, auth_middleware},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    middleware::from_fn_with_state,
    routing::*,
};
use domain::services::{ISessionService, ITableService};
use infrastructure::state::AppState;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/tables/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    responses(
    )
)]
#[axum::debug_handler]
pub async fn get_sessions(
    Path(table_id): Path<Uuid>,
    claims: ClaimsExtractor,
    State(table_service): State<Arc<dyn ITableService>>,
) -> Result<Json<Vec<GetSessionsResponse>>, HttpError> {
    /*
    let user_id = claims.0.sub;

    let table = table_service.find_by_id(table_id).await?;

    if table.is_none() {
        return Err(Error::Domain(DomainError::TableNotFound).into());
    }

    let table = table.unwrap();

    if table.owner_id() != user_id {
        return Err(Error::Application(ApplicationError::InvalidCredentials).into());
    }

    let sessions = app_state
        .session_service
        .get(GetSessionCommand {
            table_id: Some(table_id),
            ..Default::default()
        })
        .await?
        .iter()
        .map(GetSessionsResponse::from)
        .collect();

    Ok(Json(sessions))
    */
    todo!()
}

#[utoipa::path(
    post,
    path = "/v1/tables/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    request_body = CreateSessionRequest,
    responses()
)]
#[axum::debug_handler]
pub async fn create_session(
    claims: ClaimsExtractor,
    Path(table_id): Path<Uuid>,
    State(session_service): State<Arc<dyn ISessionService>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, HttpError> {
    /*
    let user_id = claims.0.sub;

    let table = session_service.create().await;

    let table = table_service.find_by_id(table_id).await?;

    let table = match table {
        Some(t) => t,
        None => return Err(Error::Domain(DomainError::TableNotFound).into()),
    };

    if table.owner_id() != user_id {
        return Err(Error::Application(ApplicationError::InvalidCredentials).into());
    }

    let session = session_service.create(todo!()).await?;

    Ok(Json(CreateSessionResponse { id: session.id }))
    */
    todo!()
}

#[utoipa::path(
    put,
    path = "/v1/tables/{table_id}/sessions/{id}",
    tag = "sessions",
    security(("auth" = [])),
    params(
        ("table_id" = Uuid, Path, description = "Table ID"),
        ("id" = Uuid, Path, description = "Session ID")
    ),
    request_body = UpdateSessionRequest,
    responses(
        (status = 200, description = "Session updated successfully", body = UpdateSessionResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to update this session", body = ErrorResponse),
        (status = 404, description = "Session not found", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_session(
    claims: ClaimsExtractor,
    State(app_app): State<Arc<dyn ISessionService>>,
    Path((table_id, session_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateSessionRequest>,
) -> Result<Json<UpdateSessionResponse>, HttpError> {
    /*
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let table = app_app.table_service.find_by_id(&table_id).await?;
    if table.gm_id != claims.0.sub {
        return Err(Error::Domain(DomainError::BusinessRuleViolation {
            message: "User is not the GM of the table".to_string(),
        }));
    }

    let session = app_app.session_service.find_by_id(&session_id).await?;

    let status = Update::from(payload.status.map(SessionStatus::from));

    let command = UpdateSessionCommand {
        id: session.id,
        title: payload.title.into(),
        description: payload.description.into(),
        scheduled_for: payload.scheduled_for.into(),
        status,
    };

    let updated_session = app_app.session_service.update(command).await?;

    Ok(Json(updated_session.into()))
    */
    todo!()
}

#[utoipa::path(
    delete,
    path = "/v1/tables/{table_id}/sessions/{id}",
    tag = "sessions",
    security(("auth" = [])),
    params(
        ("table_id" = Uuid, Path, description = "Table ID"),
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
    claims: ClaimsExtractor,
    State(session_service): State<Arc<dyn ISessionService>>,
    Path((table_id, session_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DeleteSessionResponse>, HttpError> {
    /*
    session_service
        .delete(DeleteSessionCommand {
            table_id,
            requester_id: claims.user_id(),
            session_id,
        })
        .await?;

    Ok(Json(DeleteSessionResponse {
        message: format!("Session {} deleted successfully", session_id),
    }))
    */
    todo!()
}

pub fn session_routes(state: AppState) -> Router {
    Router::new()
        .nest(
            "/tables/{table_id}/sessions",
            Router::new()
                .route("/", post(create_session))
                .route("/", get(get_sessions))
                .route("/{session_id}", delete(delete_session))
                .route("/{session_id}", put(update_session)),
        )
        .nest(
            "/sessions",
            Router::new()
                .route("/", get(get_sessions))
                .route("/{id}", put(update_session))
                .route("/{id}", delete(delete_session)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
