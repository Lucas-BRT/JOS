use crate::entities::TableStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableCommand {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub slots: u32,
    pub game_system_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTableCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub slots: Option<u32>,
    pub game_system_id: Option<Uuid>,
    pub status: Option<TableStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTableCommand {
    pub id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub status: Option<TableStatus>,
    pub game_system_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}
