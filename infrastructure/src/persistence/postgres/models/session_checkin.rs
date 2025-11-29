use crate::persistence::models::EIntentStatus;
use chrono::{DateTime, Utc};
use domain::entities::{SessionCheckin, session_checkin::SessionCheckinResult};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct SessionCheckinModel {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SessionCheckinModel> for SessionCheckin {
    fn from(model: SessionCheckinModel) -> Self {
        SessionCheckin {
            id: model.id,
            session_intent_id: model.session_intent_id,
            attendance: model.attendance,
            notes: model.notes,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionCheckinResultModel {
    pub user_id: Uuid,
    pub intent_status: EIntentStatus,
    pub attendance: bool,
    pub checkin_id: Uuid,
}

impl From<SessionCheckinResultModel> for SessionCheckinResult {
    fn from(value: SessionCheckinResultModel) -> Self {
        Self {
            user_id: value.user_id,
            intent_status: value.intent_status.into(),
            attendance: value.attendance,
            checkin_id: value.checkin_id,
        }
    }
}
