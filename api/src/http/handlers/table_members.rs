use crate::http::dtos::TableMemberResponse;
use axum::{
    Json,
    extract::{Path, State},
};
use infrastructure::state::AppState;
use shared::Result;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/tables/{table_id}/members",
    responses(
        (status = 200, description = "List of table members", body = Vec<TableMemberResponse>)
    )
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

pub fn table_members_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_table_members))
        .with_state(app_state)
}
