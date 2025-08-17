use crate::domain::{
    table::{entity::Visibility, search_filters::TableFilters},
    utils::pagination::Pagination,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateTableCommand {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: Visibility,
    pub player_slots: u32,
    pub game_system_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateTableCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<Visibility>,
    pub player_slots: Option<u32>,
    pub game_system_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct DeleteTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct GetTableCommand {
    pub filters: TableFilters,
    pub pagination: Pagination,
}
