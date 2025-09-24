use crate::domain::entities::{IntentStatus, Update};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CreateSessionIntentCommand {
    pub player_id: Uuid,
    pub session_id: Uuid,
    pub status: IntentStatus,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateSessionIntentCommand {
    pub id: Uuid,
    pub status: Update<IntentStatus>,
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteSessionIntentCommand {
    pub id: Uuid,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetSessionIntentCommand {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub status: Option<IntentStatus>,
}
