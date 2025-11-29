use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSessionCheckinCommand {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionCheckinCommand {
    pub id: Uuid,
    pub session_intent_id: Option<Uuid>,
    pub attendance: Option<bool>,
    pub notes: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSessionCheckinCommand {
    pub id: Option<Uuid>,
    pub session_intent_id: Option<Uuid>,
    pub attendance: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSessionCheckinCommand {
    pub id: Uuid,
}
