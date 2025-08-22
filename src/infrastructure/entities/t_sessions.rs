use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::session::Session;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub table_id: Uuid,
    pub accepting_intents: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        Session {
            id: model.id,
            table_id: model.table_id,
            name: model.name,
            description: model.description,
            accepting_intents: model.accepting_intents,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
