use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TableRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: TableRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum TableRequestStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}
