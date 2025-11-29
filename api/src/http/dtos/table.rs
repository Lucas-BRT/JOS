use axum::{
    Json,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain::entities::{Table, TableStatus};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateTableRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub system_id: Uuid,
    #[validate(length(max = 1000))]
    pub description: String,
    #[validate(range(min = 1, max = 20))]
    pub max_players: i32,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateTableRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub system: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    #[validate(range(min = 1, max = 20))]
    pub max_players: Option<i32>,
    pub visibility: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameMasterInfo {
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PlayerInfo {
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionInfo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub scheduled_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub enum ITableStatus {
    #[default]
    Active,
    Inactive,
}

impl From<TableStatus> for ITableStatus {
    fn from(value: TableStatus) -> Self {
        match value {
            TableStatus::Active => ITableStatus::Active,
            TableStatus::Inactive => ITableStatus::Inactive,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableListItem {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub player_slots: u32,
    pub status: TableStatus,
    pub game_system_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Table> for TableListItem {
    fn from(value: Table) -> Self {
        Self {
            id: value.id,
            gm_id: value.gm_id,
            title: value.title,
            description: value.description,
            player_slots: value.player_slots,
            status: value.status,
            game_system_id: value.game_system_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableDetails {
    pub id: Uuid,
    pub title: String,
    pub game_system: String,
    pub game_master: GameMasterInfo,
    pub description: String,
    pub player_slots: i32,
    pub players: Vec<PlayerInfo>,
    pub status: String,
    pub sessions: Vec<SessionInfo>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTableResponse {
    pub id: Uuid,
}

impl From<Table> for CreateTableResponse {
    fn from(value: Table) -> Self {
        Self { id: value.id }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteTableResponse {
    pub message: String,
}

// IntoResponse implementations
impl IntoResponse for TableDetails {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for DeleteTableResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
