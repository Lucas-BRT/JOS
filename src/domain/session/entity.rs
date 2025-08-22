use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub table_id: Uuid,
    pub name: String,
    pub description: String,
    pub accepting_intents: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
