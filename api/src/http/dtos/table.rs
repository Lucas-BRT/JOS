use axum::{
    Json,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain::entities::{Session, Table, TableDetails, TableStatus, User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::http::dtos::ISessionStatus;

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

impl From<User> for PlayerInfo {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionInfo {
    pub id: Uuid,
    pub title: String,
    pub status: ISessionStatus,
}

impl From<Session> for SessionInfo {
    fn from(value: Session) -> Self {
        Self {
            id: value.id,
            title: value.title,
            status: value.status.into(),
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

#[derive(Deserialize, Serialize, ToSchema, Debug, Default)]
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
pub struct ITableDetails {
    pub id: Uuid,
    pub title: String,
    pub game_system_id: Uuid,
    pub game_master_id: Uuid,
    pub description: String,
    pub player_slots: i32,
    pub players: Vec<PlayerInfo>,
    pub sessions: Vec<SessionInfo>,
    pub status: ITableStatus,
    pub created_at: DateTime<Utc>,
}

impl From<TableDetails> for ITableDetails {
    fn from(value: TableDetails) -> Self {
        Self {
            id: value.id,
            title: value.title,
            game_system_id: value.game_system_id,
            game_master_id: value.gm_id,
            description: value.description,
            player_slots: value.player_slots as i32,
            players: value.players.into_iter().map(PlayerInfo::from).collect(),
            sessions: value.sessions.into_iter().map(SessionInfo::from).collect(),
            status: value.status.into(),
            created_at: value.created_at,
        }
    }
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
impl IntoResponse for ITableDetails {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for DeleteTableResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
