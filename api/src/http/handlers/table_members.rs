use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{DateTime, Utc};
use domain::entities::TableMember;
use infrastructure::state::AppState;
use serde::{Deserialize, Serialize};
use shared::Result;
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ToSchema)]
pub struct TableMemberResponse {
    joined_at: DateTime<Utc>,
    user_id: Uuid,
    table_id: Uuid,
}

impl From<TableMember> for TableMemberResponse {
    fn from(value: TableMember) -> Self {
        Self {
            joined_at: value.created_at,
            user_id: value.user_id,
            table_id: value.table_id,
        }
    }
}

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
