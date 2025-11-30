use crate::http::dtos::*;
use crate::http::middleware::auth::{ClaimsExtractor, auth_middleware};
use axum::extract::*;
use axum::middleware::from_fn_with_state;
use domain::entities::commands::table_commands::*;
use domain::entities::commands::table_request_commands::*;
use infrastructure::state::AppState;
use shared::Result;
use shared::error::*;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/",
    tag = "table",
    security(("auth" = [])),
    summary = "Create a new Table"
)]
#[axum::debug_handler]
pub async fn create_table(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequest>,
) -> Result<Json<CreateTableResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = CreateTableCommand::new(
        claims.get_user_id(),
        payload.title,
        payload.description,
        payload.max_players as u32,
        payload.system_id,
    );

    let table = app_state.table_service.create_table(command).await?;

    let response = CreateTableResponse { id: table.id };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/",
    summary = "Get list of tables",
    tag = "table",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn get_tables(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TableListItem>>> {
    let tables = app_state
        .table_service
        .get_all_tables()
        .await?
        .into_iter()
        .map(|t| t.into())
        .collect::<Vec<TableListItem>>();

    Ok(Json(tables))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "table",
    summary = "Get details from a specific table",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn get_table_details(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<Option<ITableDetails>>> {
    let details = app_state
        .table_service
        .get_table_details(table_id)
        .await?
        .map(ITableDetails::from);

    Ok(Json(details))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tag = "table",
    summary = "Update a existing Table",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn update_table(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<UpdateTableRequest>,
) -> Result<Json<()>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    todo!()
}

#[utoipa::path(
    delete,
    path = "/{id}",
    summary = "Delete a existing Table",
    tag = "table",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn delete_table(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<DeleteTableResponse>> {
    app_state
        .table_service
        .delete_table(table_id, claims.get_user_id())
        .await?;

    Ok(Json(DeleteTableResponse {
        message: format!("Table {} deleted successfully", table_id),
    }))
}

#[utoipa::path(
    get,
    path = "/{table_id}/sessions",
    tag = "session",
    summary = "Get a list of sessions of a specific table",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn get_sessions(
    Path(table_id): Path<Uuid>,
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<GetSessionsResponse>>> {
    let user_id = claims.get_user_id();

    let sessions = app_state
        .session_service
        .get_table_sessions(table_id, user_id)
        .await?
        .into_iter()
        .map(GetSessionsResponse::from)
        .collect();

    Ok(Json(sessions))
}

#[utoipa::path(
    post,
    path = "/{table_id}/sessions",
    tag = "session",
    summary = "Create a new session in a existing Table",
    security(("auth" = []))
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

    let session = app_state
        .session_service
        .create_session_for_table(
            claims.get_user_id(),
            table_id,
            payload.title,
            payload.description,
            payload.scheduled_for,
        )
        .await?;

    Ok(Json(CreateSessionResponse { id: session.id }))
}

#[utoipa::path(
    get,
    path = "/{table_id}/requests",
    summary = "Get all the requests recived in a existing Table",
    tag = "table-request",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn get_received_requests(
    Path(table_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ReceivedRequestItem>>> {
    let requests = app_state.table_service.get_table_requests(table_id).await?;

    let requests = requests
        .into_iter()
        .map(|request| ReceivedRequestItem {
            id: request.id,
            player_id: request.user_id,
            table_id: request.table_id,
            request_date: request.created_at,
            message: request.message,
        })
        .collect::<Vec<ReceivedRequestItem>>();

    Ok(Json(requests))
}

#[utoipa::path(
    post,
    path = "/{table_id}/requests",
    tag = "table-request",
    security(("auth" = [])),
    summary = "Create a table join request"
)]
#[axum::debug_handler]
async fn create_request(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
    Json(payload): Json<CreateTableRequestRequest>,
) -> Result<Json<CreateTableRequestResponse>> {
    let requester_id = claims.get_user_id();

    let command = CreateTableRequestCommand::new(requester_id, table_id, payload.message);

    let table_request = app_state.table_request_service.create(command).await?;

    Ok(Json(CreateTableRequestResponse {
        id: table_request.id,
    }))
}

#[utoipa::path(get,
    path = "/{table_id}/members",
    tag = "table",
    security(("auth" = [])),
    summary = "Get all members of a specific table"
)]
async fn get_table_members(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<Vec<TableMemberResponse>>> {
    let members = app_state
        .table_member_service
        .find_by_table_id(&table_id)
        .await?;

    let members = members
        .into_iter()
        .map(TableMemberResponse::from)
        .collect::<Vec<TableMemberResponse>>();

    Ok(Json(members))
}

pub fn table_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/tables",
            OpenApiRouter::new()
                .routes(routes!(get_tables))
                .routes(routes!(create_table))
                .routes(routes!(get_table_details))
                .routes(routes!(update_table))
                .routes(routes!(delete_table))
                .routes(routes!(create_session))
                .routes(routes!(create_request))
                .routes(routes!(get_sessions))
                .routes(routes!(get_received_requests))
                .routes(routes!(get_table_members))
                .layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .with_state(state)
}
