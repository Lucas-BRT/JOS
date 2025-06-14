use crate::domain::utils::pagination::Pagination;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateTableCommand {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

pub struct UpdateTableCommand {
    pub title: String,
    pub description: String,
    pub is_public: bool,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableFilters {
    pub title: Option<String>,
    pub game_system_id: Option<Uuid>,
    pub is_public: Option<bool>,
    pub player_slots: Option<u32>,
    pub occupied_slots: Option<u32>,
    pub bg_image_link: Option<String>,
}

#[allow(unused)]
pub struct TableGetOptions {
    pagination: Option<Pagination>,
    filters: Option<TableFilters>,
}

impl TableGetOptions {
    pub fn new(pagination: Option<Pagination>, filters: Option<TableFilters>) -> Self {
        Self {
            pagination,
            filters,
        }
    }
}

impl Default for TableGetOptions {
    fn default() -> Self {
        Self {
            pagination: Some(Pagination::default()),
            filters: None,
        }
    }
}
