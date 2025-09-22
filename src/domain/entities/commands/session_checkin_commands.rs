use crate::domain::utils::update::Update;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSessionCheckinCommand {
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSessionCheckinCommand {
    pub id: Uuid,
    pub session_intent_id: Update<Uuid>,
    pub attendance: Update<bool>,
    pub notes: Update<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct DeleteSessionCheckinCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct GetSessionCheckinCommand {
    pub id: Option<Uuid>,
    pub session_intent_id: Option<Uuid>,
    pub attendance: Option<bool>,
}
