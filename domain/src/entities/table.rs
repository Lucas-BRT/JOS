use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ToSchema, Default)]
pub enum TableStatus {
    #[default]
    Active,
    Finished,
    Cancelled,
}

impl fmt::Display for TableStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TableStatus::Active => write!(f, "Active"),
            TableStatus::Finished => write!(f, "Finished"),
            TableStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl FromStr for TableStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(TableStatus::Active),
            "Finished" => Ok(TableStatus::Finished),
            "Cancelled" => Ok(TableStatus::Cancelled),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub player_slots: u32,
    pub status: TableStatus,
    pub game_system_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
