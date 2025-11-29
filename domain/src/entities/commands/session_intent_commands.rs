use crate::entities::IntentStatus;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub struct CreateSessionIntentCommand {
    pub id: Uuid,
    pub player_id: Uuid,
    pub session_id: Uuid,
    pub status: IntentStatus,
}

impl CreateSessionIntentCommand {
    pub fn new(player_id: Uuid, session_id: Uuid, status: IntentStatus) -> Self {
        Self {
            id: Uuid::now_v7(),
            player_id,
            session_id,
            status,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSessionIntentCommand {
    pub id: Uuid,
    pub status: Option<IntentStatus>,
}

#[derive(Debug, Clone, Copy, Default)]
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
