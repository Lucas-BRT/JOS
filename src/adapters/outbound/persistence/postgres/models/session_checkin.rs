use crate::domain::entities::SessionCheckin;
use crate::shared::Date;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct SessionCheckinModel {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
    pub created_at: Date,
    pub updated_at: Date,
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
