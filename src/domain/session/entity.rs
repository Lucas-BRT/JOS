use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub status: SessionStatus,
    pub accepting_intents: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SessionStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}
