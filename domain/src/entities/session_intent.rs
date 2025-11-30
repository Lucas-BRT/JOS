use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SessionIntent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub intent_status: IntentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IntentStatus {
    #[default]
    Unsure,
    Confirmed,
    Declined,
}
