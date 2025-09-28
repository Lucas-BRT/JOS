use crate::domain::entities::{TableRequest, TableRequestStatus};
use uuid::Uuid;
use crate::shared::Date;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "request_status", rename_all = "lowercase")]
pub enum ETableRequestStatus {
    Pending,
    Approved,
    Rejected,
}

impl From<TableRequestStatus> for ETableRequestStatus {
    fn from(status: TableRequestStatus) -> Self {
        match status {
            TableRequestStatus::Pending => ETableRequestStatus::Pending,
            TableRequestStatus::Approved => ETableRequestStatus::Approved,
            TableRequestStatus::Rejected => ETableRequestStatus::Rejected,
        }
    }
}

impl From<ETableRequestStatus> for TableRequestStatus {
    fn from(status: ETableRequestStatus) -> Self {
        match status {
            ETableRequestStatus::Pending => TableRequestStatus::Pending,
            ETableRequestStatus::Approved => TableRequestStatus::Approved,
            ETableRequestStatus::Rejected => TableRequestStatus::Rejected,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableRequestModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: ETableRequestStatus,
    pub created_at: Date,
    pub updated_at: Date,
}

impl From<TableRequestModel> for TableRequest {
    fn from(model: TableRequestModel) -> Self {
        TableRequest {
            id: model.id,
            user_id: model.user_id,
            table_id: model.table_id,
            message: model.message,
            status: model.status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
