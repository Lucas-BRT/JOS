use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTableMemberCommand {
    pub id: Uuid,
    pub table_id: Uuid,
    pub user_id: Uuid,
}

impl CreateTableMemberCommand {
    pub fn new(table_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            table_id,
            user_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTableMemberCommand {
    pub id: Uuid,
    pub table_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTableMemberCommand {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteTableMemberCommand {
    pub id: Uuid,
}
