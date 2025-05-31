use crate::{core::state::AppState, domain::table::entity::Table, prelude::AppResult};
use axum::{Json, Router, extract::State, routing::get};

pub async fn get_available_tables(
    State(app_state): State<AppState>,
) -> AppResult<Json<Vec<Table>>> {
    let users = app_state.table_service.get_avaliable().await?;

    Ok(Json(users))
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_available_tables))
        .with_state(state.clone())
}
