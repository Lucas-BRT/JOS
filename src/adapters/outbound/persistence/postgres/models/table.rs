use crate::domain::table::entity::{Table, TableStatus, Visibility};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "table_visibility", rename_all = "lowercase")]
pub enum ETableVisibility {
    Private,
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "table_status", rename_all = "lowercase")]
pub enum ETableStatus {
    Draft,
    Open,
    Paused,
    Completed,
    Cancelled,
}

impl From<ETableVisibility> for Visibility {
    fn from(value: ETableVisibility) -> Self {
        match value {
            ETableVisibility::Private => Visibility::Private,
            ETableVisibility::Public => Visibility::Public,
        }
    }
}

impl From<Visibility> for ETableVisibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Private => ETableVisibility::Private,
            Visibility::Public => ETableVisibility::Public,
        }
    }
}

impl From<ETableStatus> for TableStatus {
    fn from(value: ETableStatus) -> Self {
        match value {
            ETableStatus::Draft => TableStatus::Draft,
            ETableStatus::Open => TableStatus::Open,
            ETableStatus::Paused => TableStatus::Paused,
            ETableStatus::Completed => TableStatus::Completed,
            ETableStatus::Cancelled => TableStatus::Cancelled,
        }
    }
}

impl From<TableStatus> for ETableStatus {
    fn from(value: TableStatus) -> Self {
        match value {
            TableStatus::Draft => ETableStatus::Draft,
            TableStatus::Open => ETableStatus::Open,
            TableStatus::Paused => ETableStatus::Paused,
            TableStatus::Completed => ETableStatus::Completed,
            TableStatus::Cancelled => ETableStatus::Cancelled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableModel {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: ETableVisibility,
    pub player_slots: i32,
    pub game_system_id: Uuid,
    pub status: ETableStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<TableModel> for Table {
    fn from(model: TableModel) -> Self {
        Table {
            id: model.id,
            gm_id: model.gm_id,
            title: model.title,
            description: model.description,
            visibility: model.visibility.into(),
            player_slots: model.player_slots as u32,
            game_system_id: model.game_system_id,
            status: model.status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
