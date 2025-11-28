use crate::entities::TableRequestStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTableRequestCommand {
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
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
