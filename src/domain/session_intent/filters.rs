use super::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SessionIntentFilter {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub intent_status: Option<IntentStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SessionIntentFilter {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        session_id: Uuid,
        intent_status: IntentStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Some(id),
            user_id: Some(user_id),
            session_id: Some(session_id),
            intent_status: Some(intent_status),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }

    pub fn with_id(self, id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn with_user_id(self, user_id: Uuid) -> Self {
        Self {
            user_id: Some(user_id),
            ..self
        }
    }

    pub fn with_session_id(self, session_id: Uuid) -> Self {
        Self {
            session_id: Some(session_id),
            ..self
        }
    }

    pub fn with_intent_status(self, intent_status: IntentStatus) -> Self {
        Self {
            intent_status: Some(intent_status),
            ..self
        }
    }

    pub fn with_created_at(self, created_at: DateTime<Utc>) -> Self {
        Self {
            created_at: Some(created_at),
            ..self
        }
    }

    pub fn with_updated_at(self, updated_at: DateTime<Utc>) -> Self {
        Self {
            updated_at: Some(updated_at),
            ..self
        }
    }
}
