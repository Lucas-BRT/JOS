use chrono::{DateTime, Utc};
use domain::entities::GameSystem;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct GameSystemModel {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GameSystemModel> for GameSystem {
    fn from(model: GameSystemModel) -> Self {
        GameSystem {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
