use super::dtos::{CreateTableRequestDto, TableRequestResponse, UpdateTableRequestDto};
use crate::{
    core::state::AppState, domain::{
        auth::Claims,
        table_request::dtos::{CreateTableRequestCommand, DeleteTableRequestCommand, TableRequestFilters, UpdateTableRequestCommand}, utils::pagination::Pagination,
    }, Result
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/table-requests",
    tag = "table-requests",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateTableRequestDto,
    responses(
        (status = 201, description = "Request created successfully", body = String),
        (status = 400, description = "Bad request", body = Value),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn create_table_request(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateTableRequestDto>,
) -> Result<Json<String>> {
    let mut request = CreateTableRequestCommand::from_dto(payload, user.sub);
    let request = app_state.table_request_service.create(&mut request).await?;

    Ok(Json(request.id.to_string()))
}

#[utoipa::path(
    get,
    path = "/v1/table-requests",
    tag = "table-requests",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of table requests", body = Vec<TableRequestResponse>),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn get_table_requests(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TableRequestResponse>>> {
    let requests = app_state
        .table_request_service
        .get(&TableRequestFilters::default(), Pagination::default())
        .await?;
    let requests = requests.iter().map(TableRequestResponse::from).collect();

    Ok(Json(requests))
}

#[utoipa::path(
    get,
    path = "/v1/table-requests/table/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of table requests of a table", body = Vec<TableRequestResponse>),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn get_table_requests_by_table_id(
    user: Claims,
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<Uuid>,
) -> Result<Json<Vec<TableRequestResponse>>> {

    let requests = app_state
        .table_request_service
        .get_requests_by_table_id(&table_id, &user.sub)
        .await?
        .iter()
        .map(TableRequestResponse::from)
        .collect();

    Ok(Json(requests))
}

#[utoipa::path(
    put,
    path = "/v1/table-requests/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table request ID")
    ),
    request_body = UpdateTableRequestDto,
    responses(
        (status = 200, description = "Table request updated successfully", body = ()),
        (status = 400, description = "Bad request", body = Value),
        (status = 404, description = "Table request not found", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn update_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    Json(update_payload): Json<UpdateTableRequestDto>,
) -> Result<()> {
    let update_command = UpdateTableRequestCommand {
        status: update_payload.status,
    };
    app_state
        .table_request_service
        .update(&update_command)
        .await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/v1/table-requests/{id}",
    tag = "table-requests",
    params(
        ("id" = Uuid, Path, description = "Table request ID")
    ),
    responses(
        (status = 200, description = "Table request deleted successfully", body = ()),
        (status = 404, description = "Table request not found", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn delete_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    user: Claims,
) -> Result<()> {
    let command = DeleteTableRequestCommand {
        id: request_id,
        gm_id: user.sub,
    };
    app_state.table_request_service.delete(&command).await?;

    Ok(())
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_table_requests))
        .route("/", post(create_table_request))
        .route("/{id}", put(update_table_request))
        .route("/{id}", delete(delete_table_request))
        .with_state(state.clone())
}
