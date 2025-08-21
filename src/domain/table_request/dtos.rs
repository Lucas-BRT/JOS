use crate::domain::{
    table_request::entity::TableRequestStatus,
    utils::{pagination::Pagination, update::Update},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CreateTableRequestCommand {
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
}

impl CreateTableRequestCommand {
    pub fn new(user_id: Uuid, table_id: Uuid, message: Option<String>) -> Self {
        Self {
            user_id,
            table_id,
            message,
        }
    }
}

pub struct UpdateTableRequestCommand {
    pub id: Uuid,
    pub status: Update<TableRequestStatus>,
    pub message: Update<Option<String>>,
}

impl UpdateTableRequestCommand {
    pub fn new(
        id: Uuid,
        status: Update<TableRequestStatus>,
        message: Update<Option<String>>,
    ) -> Self {
        Self {
            id,
            status,
            message,
        }
    }
}

pub struct DeleteTableRequestCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableRequestCommand {
    pub filters: TableRequestFilters,
    pub pagination: Pagination,
}

impl GetTableRequestCommand {
    pub fn new(filters: TableRequestFilters, pagination: Pagination) -> Self {
        Self {
            filters,
            pagination,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableRequestFilters {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<TableRequestStatus>,
}
