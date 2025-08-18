use crate::domain::game_system::GameSystem;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Model> for GameSystem {
    fn from(model: Model) -> Self {
        GameSystem {
            id: model.id,
            name: model.name,
        }
    }
}
