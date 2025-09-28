use crate::domain::entities::GameSystem;
use crate::shared::Date;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct GameSystemModel {
    pub id: Uuid,
    pub name: String,
    pub created_at: Date,
    pub updated_at: Date,
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
