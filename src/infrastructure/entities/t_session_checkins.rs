use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
