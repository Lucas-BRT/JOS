use crate::entities::SessionStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSessionCommand {
    pub table_id: Uuid,
    pub title: String,
    pub description: String,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSessionCommand {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub status: Option<SessionStatus>,
    pub scheduled_before: Option<DateTime<Utc>>,
    pub scheduled_after: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSessionCommand {
    pub id: Uuid,
}
