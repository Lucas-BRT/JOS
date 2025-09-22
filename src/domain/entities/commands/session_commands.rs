use crate::domain::entities::SessionStatus;
use crate::domain::utils::update::Update;
use crate::shared::Date;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSessionCommand {
    pub table_id: Uuid,
    pub name: String,
    pub description: String,
    pub scheduled_for: Option<Date>,
    pub status: SessionStatus,
    pub accepting_intents: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GetSessionCommand {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub name: Option<String>,
    pub status: Option<SessionStatus>,
    pub scheduled_for_start: Option<Date>,
    pub scheduled_for_end: Option<Date>,
    pub accepting_intents: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct UpdateSessionCommand {
    pub id: Uuid,
    pub name: Update<String>,
    pub description: Update<String>,
    pub scheduled_for: Update<Option<Date>>,
    pub status: Update<SessionStatus>,
    pub accepting_intents: Update<bool>,
}

#[derive(Debug, Clone)]
pub struct DeleteSessionCommand {
    pub id: Uuid,
}
