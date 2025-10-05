use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_for: Option<Date>,
    pub status: SessionStatus,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SessionStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}
