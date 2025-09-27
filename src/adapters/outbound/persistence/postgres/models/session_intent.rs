use crate::domain::entities::{IntentStatus, SessionIntent};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "intent_status", rename_all = "lowercase")]
pub enum EIntentStatus {
    Confirmed,
    Unsure,
    Declined,
}

impl From<EIntentStatus> for IntentStatus {
    fn from(status: EIntentStatus) -> Self {
        match status {
            EIntentStatus::Confirmed => IntentStatus::Confirmed,
            EIntentStatus::Unsure => IntentStatus::Tentative,
            EIntentStatus::Declined => IntentStatus::Declined,
        }
    }
}

impl From<IntentStatus> for EIntentStatus {
    fn from(status: IntentStatus) -> Self {
        match status {
            IntentStatus::Confirmed => EIntentStatus::Confirmed,
            IntentStatus::Tentative => EIntentStatus::Unsure,
            IntentStatus::Declined => EIntentStatus::Declined,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct SessionIntentModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub intent_status: EIntentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SessionIntentModel> for SessionIntent {
    fn from(model: SessionIntentModel) -> Self {
        SessionIntent {
            id: model.id,
            user_id: model.user_id,
            session_id: model.session_id,
            intent_status: model.intent_status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
