use crate::entities::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateTableCommand {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub slots: u32,
    pub game_system_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTableCommand {
    pub id: Uuid,
    pub title: Update<String>,
    pub description: Update<String>,
    pub slots: Update<u32>,
    pub game_system_id: Update<Uuid>,
}

#[derive(Debug, Clone)]
pub struct DeleteTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableCommand {
    pub id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub title: Option<String>,
    pub game_system_id: Option<Uuid>,
    pub slots: Option<u32>,
}
