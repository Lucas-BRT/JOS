use crate::entities::TableStatus;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CreateTableCommand<'a> {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub slots: u32,
    pub game_system_id: Uuid,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateTableCommand<'a> {
    pub id: Uuid,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub slots: Option<u32>,
    pub game_system_id: Option<Uuid>,
    pub status: Option<TableStatus>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DeleteTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetTableCommand<'a> {
    pub id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub title: Option<&'a str>,
    pub game_system_id: Option<Uuid>,
    pub slots: Option<u32>,
    pub search_term: Option<&'a str>,
}
