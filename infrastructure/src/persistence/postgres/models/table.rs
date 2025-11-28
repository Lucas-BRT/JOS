use chrono::{DateTime, Utc};
use domain::entities::{Table, TableStatus};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "table_status", rename_all = "lowercase")]
pub enum ETableStatus {
    Active,
    Inactive,
}

impl From<TableStatus> for ETableStatus {
    fn from(status: TableStatus) -> Self {
        match status {
            TableStatus::Active => ETableStatus::Active,
            TableStatus::Inactive => ETableStatus::Inactive,
        }
    }
}

impl From<ETableStatus> for TableStatus {
    fn from(status: ETableStatus) -> Self {
        match status {
            ETableStatus::Active => TableStatus::Active,
            ETableStatus::Inactive => TableStatus::Inactive,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableModel {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: ETableStatus,
    pub slots: i32,
    pub game_system_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TableModel> for Table {
    fn from(model: TableModel) -> Self {
        Table {
            id: model.id,
            gm_id: model.gm_id,
            title: model.title,
            description: model.description,
            player_slots: model.slots as u32,
            status: model.status.into(),
            game_system_id: model.game_system_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
