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
    get,
    path = "/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = Vec<SessionListItem>),
        (status = 401, description = "Authentication required", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_sessions(
    Path(table_id): Path<Uuid>,
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<GetSessionsResponse>>> {
    let user_id = claims.0.sub;

    let table = app_state.table_service.find_by_id(&table_id).await?;

    if table.gm_id != user_id {
        return Err(Error::Application(ApplicationError::InvalidCredentials));
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
}

#[utoipa::path(
    post,
    path = "/{table_id}/sessions",
    tag = "sessions",
    security(("auth" = [])),
    request_body = CreateSessionRequest,
    responses(
        (status = 201, description = "Session created successfully", body = SessionDetails),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Not authorized to create session for this table", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_session(
    claims: ClaimsExtractor,
    Path(table_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let user_id = claims.0.sub;

    let table = app_state.table_service.find_by_id(&table_id).await?;

    if table.gm_id != user_id {
        return Err(Error::Application(ApplicationError::InvalidCredentials));
    }

    let session = app_state
        .session_service
        .create(
            user_id,
            CreateSessionCommand {
                table_id,
                title: payload.title,
                description: payload.description,
                scheduled_for: payload.scheduled_for,
                status: payload.status.unwrap_or_default(),
            },
        )
        .await?;

    Ok(Json(CreateSessionResponse { id: session.id }))
}

#[utoipa::path(
    put,
    path = "/{table_id}/sessions/{id}",
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
    State(app_state): State<Arc<AppState>>,
    Path((table_id, session_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateSessionRequest>,
) -> Result<Json<UpdateSessionResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let table = app_state.table_service.find_by_id(&table_id).await?;
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
    path = "/{id}",
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
    State(app_state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<DeleteSessionResponse>> {
    let table = app_state
        .table_service
        .find_by_session_id(&session_id)
        .await?;

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

pub fn session_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/tables/{table_id}/sessions", OpenApiRouter::new())
        .nest(
            "/sessions",
            OpenApiRouter::new()
                .routes(routes!(get_sessions))
                .routes(routes!(update_session))
                .routes(routes!(delete_session)),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
