use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub visibility: Visibility,
    pub player_slots: u32,
    pub game_system_id: Uuid,
    pub status: TableStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum TableStatus {
    Draft,
    Open,
    Paused,
    Completed,
    Cancelled,
}
