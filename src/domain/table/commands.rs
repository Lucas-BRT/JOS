use crate::domain::{
    table::{entity::Visibility, search_filters::TableFilters},
    utils::{pagination::Pagination, update::Update},
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

#[derive(Debug, Clone, Default)]
pub struct UpdateTableCommand {
    pub id: Uuid,
    pub title: Update<String>,
    pub description: Update<String>,
    pub visibility: Update<Visibility>,
    pub player_slots: Update<u32>,
    pub game_system_id: Update<Uuid>,
}

#[derive(Debug, Clone)]
pub struct DeleteTableCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableCommand {
    pub filters: TableFilters,
    pub pagination: Pagination,
}

impl GetTableCommand {
    pub fn new(filters: TableFilters, pagination: Pagination) -> Self {
        Self {
            filters,
            pagination,
        }
    }

    pub fn with_pagination(self, pagination: Pagination) -> Self {
        Self {
            filters: self.filters,
            pagination,
        }
    }

    pub fn with_filters(self, filters: TableFilters) -> Self {
        Self {
            filters,
            pagination: self.pagination,
        }
    }
}
