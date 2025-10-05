use crate::entities::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateTableMemberCommand {
    pub table_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTableMemberCommand {
    pub id: Uuid,
    pub table_id: Update<Uuid>,
    pub user_id: Update<Uuid>,
}

#[derive(Debug, Clone)]
pub struct DeleteTableMemberCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableMemberCommand {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}
