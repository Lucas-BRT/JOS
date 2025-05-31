use super::dtos::CreateTableDto;
use crate::{core::state::AppState, domain::table::entity::Table, prelude::AppResult};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

pub async fn create_table(
    State(app_state): State<AppState>,
    Json(new_table_payload): Json<CreateTableDto>,
) -> AppResult<Json<String>> {
    let users = app_state
        .table_service
        .create_table(&new_table_payload)
        .await?;

    Ok(Json(users))
}

pub async fn get_available_tables(
    State(app_state): State<AppState>,
) -> AppResult<Json<Vec<Table>>> {
    let users = app_state.table_service.get_avaliable().await?;

    Ok(Json(users))
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .route("/", post(create_table))
        .with_state(state.clone())
}
