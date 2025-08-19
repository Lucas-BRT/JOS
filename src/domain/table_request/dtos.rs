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

pub struct DeleteTableRequestCommand {
    pub id: Uuid,
    pub gm_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableRequestFilters {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<String>,
}
