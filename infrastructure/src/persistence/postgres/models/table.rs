use domain::entities::Table;
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableModel {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub slots: i32,
    pub game_system_id: Uuid,
    pub created_at: Date,
    pub updated_at: Date,
}

impl From<TableModel> for Table {
    fn from(model: TableModel) -> Self {
        Table {
            id: model.id,
            gm_id: model.gm_id,
            title: model.title,
            description: model.description,
            player_slots: model.slots as u32,
            game_system_id: model.game_system_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
