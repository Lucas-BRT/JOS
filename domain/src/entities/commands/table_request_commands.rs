use crate::entities::TableRequestStatus;
use uuid::Uuid;

pub struct CreateTableRequestCommand<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<&'a str>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTableRequestCommand<'a> {
    pub id: Uuid,
    pub status: Option<TableRequestStatus>,
    pub message: Option<Option<&'a str>>,
}

pub struct DeleteTableRequestCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetTableRequestCommand {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<TableRequestStatus>,
}
