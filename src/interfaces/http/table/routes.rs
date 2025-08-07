use super::dtos::CreateTableDto;
use crate::{
    Result, core::state::AppState, domain::table::dtos::CreateTableCommand,
    interfaces::http::{table::dtos::AvaliableTableResponse, openapi::{schemas::*, tags::TABLE_TAG}},
};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use std::sync::Arc;
use utoipa::OpenApi;

/// Create a new RPG table
#[utoipa::path(
    post,
    path = "/tables",
    tag = TABLE_TAG,
    request_body = CreateTableRequest,
    responses(
        (status = 201, description = "Table created successfully", body = IdResponse),
        (status = 400, description = "Validation error", body = ValidationErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn create_table(
    State(app_state): State<Arc<AppState>>,
    Json(new_table_payload): Json<CreateTableDto>,
) -> Result<Json<String>> {
    let table = CreateTableCommand::from(new_table_payload);
    let table_id = app_state.table_service.create(&table).await?;

    Ok(Json(table_id))
}

/// Get all available RPG tables
#[utoipa::path(
    get,
    path = "/tables",
    tag = TABLE_TAG,
    responses(
        (status = 200, description = "List of available tables", body = Vec<AvailableTableResponse>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn get_available_tables(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let tables = app_state.table_service.get().await?;

    let tables = tables.iter().map(AvaliableTableResponse::from).collect();

    Ok(Json(tables))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .with_state(state.clone())
}
