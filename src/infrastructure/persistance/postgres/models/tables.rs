use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub system_id: u32,
    pub contact_info: String,
    pub max_players: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
