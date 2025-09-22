use crate::domain::{entities::TableRequestStatus, utils::update::Update};
use uuid::Uuid;

pub struct CreateTableRequestCommand {
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
}

pub struct UpdateTableRequestCommand {
    pub table_id: Uuid,
    pub status: Update<TableRequestStatus>,
    pub message: Update<Option<String>>,
}

pub struct DeleteTableRequestCommand {
    pub table_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableRequestCommand {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<TableRequestStatus>,
}
