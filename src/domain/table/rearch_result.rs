use super::{game_genre::GameGenre, title::Title};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSearchResult {
    pub id: Uuid,
    pub title: Title,
    pub gm_display_name: String,
    pub description_snippet: Option<String>,
    pub system_name: String,
    pub genre_names: Vec<GameGenre>,
    pub max_players: Option<i32>,
    pub current_players_count: u32,
    pub contact_info_summary: String,
    pub created_at: DateTime<Utc>,
}
