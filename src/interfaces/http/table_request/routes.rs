use super::dtos::{CreateTableRequestDto, TableRequestResponse, UpdateTableRequestDto};
use crate::{
    Result, core::state::AppState, domain::table_request::dtos::CreateTableRequestCommand,
    interfaces::http::openapi::{schemas::*, tags::TABLE_REQUEST_TAG},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post, put, delete},
};
use std::sync::Arc;
use utoipa::OpenApi;
use uuid::Uuid;

/// Create a new table request
#[utoipa::path(
    post,
    path = "/table-requests",
    tag = TABLE_REQUEST_TAG,
    request_body = CreateTableRequestRequest,
    responses(
        (status = 201, description = "Table request created successfully", body = IdResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_table_request(
    State(app_state): State<Arc<AppState>>,
    Json(new_request_payload): Json<CreateTableRequestDto>,
) -> Result<Json<String>> {
    let request = CreateTableRequestCommand::from(new_request_payload);
    let request_id = app_state.table_request_service.create(&request).await?;

    Ok(Json(request_id))
}

/// Get all table requests
#[utoipa::path(
    get,
    path = "/table-requests",
    tag = TABLE_REQUEST_TAG,
    responses(
        (status = 200, description = "List of table requests", body = Vec<TableRequestResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_table_requests(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TableRequestResponse>>> {
    let requests = app_state.table_request_service.get().await?;

    let requests = requests.iter().map(TableRequestResponse::from).collect();

    Ok(Json(requests))
}

/// Get a specific table request by ID
#[utoipa::path(
    get,
    path = "/table-requests/{id}",
    tag = TABLE_REQUEST_TAG,
    params(
        ("id" = String, Path, description = "Table request ID")
    ),
    responses(
        (status = 200, description = "Table request found", body = TableRequestResponse),
        (status = 404, description = "Table request not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_table_request_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<Option<TableRequestResponse>>> {
    let request = app_state.table_request_service.find_by_id(&request_id).await?;

    let response = request.as_ref().map(TableRequestResponse::from);

    Ok(Json(response))
}

/// Update a table request status
#[utoipa::path(
    put,
    path = "/table-requests/{id}",
    tag = TABLE_REQUEST_TAG,
    params(
        ("id" = String, Path, description = "Table request ID")
    ),
    request_body = UpdateTableRequestRequest,
    responses(
        (status = 200, description = "Table request updated successfully", body = SuccessResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse),
        (status = 404, description = "Table request not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
    Json(update_payload): Json<UpdateTableRequestDto>,
) -> Result<Json<()>> {
    let update_command = crate::domain::table_request::dtos::UpdateTableRequestCommand {
        status: update_payload.status,
    };
    app_state.table_request_service.update(&request_id, &update_command).await?;

    Ok(Json(()))
}

/// Delete a table request
#[utoipa::path(
    delete,
    path = "/table-requests/{id}",
    tag = TABLE_REQUEST_TAG,
    params(
        ("id" = String, Path, description = "Table request ID")
    ),
    responses(
        (status = 200, description = "Table request deleted successfully", body = SuccessResponse),
        (status = 404, description = "Table request not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn delete_table_request(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<()>> {
    app_state.table_request_service.delete(&request_id).await?;

    Ok(Json(()))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_table_requests))
        .route("/", post(create_table_request))
        .route("/{id}", get(get_table_request_by_id))
        .route("/{id}", put(update_table_request))
        .route("/{id}", delete(delete_table_request))
        .with_state(state.clone())
}
