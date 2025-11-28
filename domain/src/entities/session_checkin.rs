use crate::entities::{IntentStatus, Session};
use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SessionCheckin {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCheckinData {
    pub user_id: Uuid,
    pub attended: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionCheckinResult {
    pub user_id: Uuid,
    pub intent_status: IntentStatus,
    pub attended: bool,
    pub checkin_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SessionFinalizationResult {
    pub session: Session,
    pub checkins: Vec<SessionCheckinResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionFinalizationData {
    pub session_id: Uuid,
    pub checkins: Vec<SessionCheckinData>,
}
