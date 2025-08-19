use super::enums::ETableVisibility;
use crate::domain::table::entity::Table;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub visibility: ETableVisibility,
    pub description: String,
    pub game_system_id: Uuid,
    pub player_slots: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for Table {
    fn from(model: Model) -> Self {
        Table {
            id: model.id,
            gm_id: model.gm_id,
            title: model.title,
            visibility: model.visibility.into(),
            player_slots: model.player_slots as u32,
            description: model.description,
            game_system_id: model.game_system_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
