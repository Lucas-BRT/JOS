use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionIntent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub intent_status: IntentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SessionIntent {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        session_id: Uuid,
        intent_status: IntentStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        SessionIntent {
            id,
            user_id,
            session_id,
            intent_status,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentStatus {
    Confirmed,
    Tentative,
    Declined,
}
