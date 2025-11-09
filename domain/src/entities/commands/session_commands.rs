use crate::entities::SessionStatus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSessionCommand<'a> {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub table_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Default)]
pub struct GetSessionCommand {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub title: Option<String>,
    pub status: Option<SessionStatus>,
    pub scheduled_for_start: Option<DateTime<Utc>>,
    pub scheduled_for_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateSessionCommand<'a> {
    pub id: Uuid,
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub scheduled_for: Option<Option<DateTime<Utc>>>,
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone)]
pub struct DeleteSessionCommand {
    pub table_id: Uuid,
    pub session_id: Uuid,
    pub requester_id: Uuid,
}
