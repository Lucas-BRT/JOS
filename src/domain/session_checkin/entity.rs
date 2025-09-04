use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SessionCheckin {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
