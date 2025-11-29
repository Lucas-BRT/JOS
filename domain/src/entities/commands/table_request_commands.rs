use crate::entities::TableRequestStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTableRequestCommand {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: TableRequestStatus,
}

impl CreateTableRequestCommand {
    pub fn new(user_id: Uuid, table_id: Uuid, message: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            table_id,
            message,
            status: TableRequestStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTableRequestCommand {
    pub id: Uuid,
    pub status: Option<TableRequestStatus>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTableRequestCommand {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<TableRequestStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteTableRequestCommand {
    pub id: Uuid,
}
