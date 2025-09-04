use crate::domain::game_system::GameSystem;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct GameSystemModel {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<GameSystemModel> for GameSystem {
    fn from(model: GameSystemModel) -> Self {
        GameSystem {
            id: model.id,
            name: model.name,
        }
    }
}
