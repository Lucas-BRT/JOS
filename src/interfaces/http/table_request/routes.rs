use super::dtos::{CreateTableRequestDto, TableRequestResponse, UpdateTableRequestDto};
use crate::{
    Result, core::state::AppState, domain::table_request::dtos::CreateTableRequestCommand,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post, put, delete},
};
use std::sync::Arc;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_table_request(
    State(app_state): State<Arc<AppState>>,
    Json(new_request_payload): Json<CreateTableRequestDto>,
) -> Result<Json<String>> {
    let request = CreateTableRequestCommand::from(new_request_payload);
    let request_id = app_state.table_request_service.create(&request).await?;

    Ok(Json(request_id))
}

#[axum::debug_handler]
pub async fn get_table_requests(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TableRequestResponse>>> {
    let requests = app_state.table_request_service.get().await?;

    let requests = requests.iter().map(TableRequestResponse::from).collect();

    Ok(Json(requests))
}

#[axum::debug_handler]
pub async fn get_table_request_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Json<Option<TableRequestResponse>>> {
    let request = app_state.table_request_service.find_by_id(&request_id).await?;

    let response = request.as_ref().map(TableRequestResponse::from);

    Ok(Json(response))
}

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
        .route("/:id", get(get_table_request_by_id))
        .route("/:id", put(update_table_request))
        .route("/:id", delete(delete_table_request))
        .with_state(state.clone())
}
