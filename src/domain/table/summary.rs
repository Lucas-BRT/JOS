use super::title::Title;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSummary {
    pub id: Uuid,
    pub title: Title,
    pub system_name: String,
    pub max_players: Option<u32>,
    pub current_players_count: u32,
    pub language: String,
    pub created_at: DateTime<Utc>,
}
