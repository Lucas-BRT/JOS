use crate::shared::Date;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionIntent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub intent_status: IntentStatus,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentStatus {
    Confirmed,
    Tentative,
    Declined,
}
