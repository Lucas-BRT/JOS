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

pub async fn create_table(
    State(app_state): State<AppState>,
    Json(new_table_payload): Json<CreateTableDto>,
) -> Result<Json<String>> {
    let table = CreateTableCommand::from(new_table_payload);
    let users = app_state.table_service.create(&table).await?;

    Ok(Json(users))
}

pub async fn get_available_tables(
    State(app_state): State<AppState>,
    Json(filters): Json<Option<TableFilters>>,
) -> Result<Json<Vec<AvaliableTableResponse>>> {
    let users = app_state.table_service.get(filters).await?;

    Ok(Json(users))
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .with_state(state.clone())
}
