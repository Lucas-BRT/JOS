use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateTableRequestCommand {
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
}

pub struct UpdateTableRequestCommand {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableRequestFilters {
    pub user_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<String>,
}

#[allow(unused)]
pub struct TableRequestGetOptions {
    pagination: Option<crate::domain::utils::pagination::Pagination>,
    filters: Option<TableRequestFilters>,
}

impl TableRequestGetOptions {
    pub fn new(
        pagination: Option<crate::domain::utils::pagination::Pagination>,
        filters: Option<TableRequestFilters>,
    ) -> Self {
        Self {
            pagination,
            filters,
        }
    }
}

impl Default for TableRequestGetOptions {
    fn default() -> Self {
        Self {
            pagination: Some(crate::domain::utils::pagination::Pagination::default()),
            filters: None,
        }
    }
}
