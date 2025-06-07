use super::dtos::CreateTableDto;
use crate::{
    Result,
    core::state::AppState,
    domain::table::dtos::{CreateTableCommand, TableFilters},
    interfaces::http::table::dtos::AvaliableTableResponse,
};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn create_table(
    State(app_state): State<Arc<AppState>>,
    Json(new_table_payload): Json<CreateTableDto>,
) -> Result<Json<String>> {
    let table = CreateTableCommand::from(new_table_payload);
    let table_id = app_state.table_service.create(&table).await?;

    Ok(Json(table_id))
}

#[axum::debug_handler]
pub async fn get_available_tables(
    State(app_state): State<Arc<AppState>>,
    Json(filters): Json<Option<TableFilters>>,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let tables = app_state.table_service.get(filters).await?;

    let tables = tables
        .iter()
        .map(|table| AvaliableTableResponse::from(table))
        .collect();

    Ok(Json(tables))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .with_state(state.clone())
}
