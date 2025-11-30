use crate::entities::TableStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TableStatus,
    pub slots: u32,
    pub game_system_id: Uuid,
}

impl CreateTableCommand {
    pub fn new(
        gm_id: Uuid,
        title: String,
        description: String,
        slots: u32,
        game_system_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            gm_id,
            title,
            description,
            slots,
            status: TableStatus::default(),
            game_system_id,
        }
    }
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
