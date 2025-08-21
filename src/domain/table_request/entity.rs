use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TableRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: TableRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, utoipa::ToSchema)]
pub enum TableRequestStatus {
    Pending,
    Approved,
    Rejected,
}

impl From<String> for TableRequestStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "pending" => TableRequestStatus::Pending,
            "approved" => TableRequestStatus::Approved,
            "rejected" => TableRequestStatus::Rejected,
            _ => TableRequestStatus::Pending,
        }
    }
}

impl From<TableRequestStatus> for String {
    fn from(status: TableRequestStatus) -> Self {
        match status {
            TableRequestStatus::Pending => "pending".to_string(),
            TableRequestStatus::Approved => "approved".to_string(),
            TableRequestStatus::Rejected => "rejected".to_string(),
        }
    }
}
