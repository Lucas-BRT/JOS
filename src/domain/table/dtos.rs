use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{games::game_genre::GameGenreVo, utils::contact_info::ContactInfoVo};

use super::vo::{DescriptionVo, TitleVo};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UpdateTableData {
    pub title: Option<TitleVo>,
    pub description: Option<Option<DescriptionVo>>,
    pub system_id: Option<u32>,
    pub contact_info: Option<ContactInfoVo>,
    pub max_players: Option<Option<u32>>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSummary {
    pub id: Uuid,
    pub title: TitleVo,
    pub system_name: String,
    pub max_players: Option<u32>,
    pub current_players_count: u32,
    pub language: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableSearchFilters {
    pub query_text: Option<String>,
    pub system_id: Option<i32>,
    pub genre_ids: Option<Vec<i32>>,
    pub has_vacancies: Option<bool>,
}

impl Default for TableSearchFilters {
    fn default() -> Self {
        Self {
            query_text: None,
            system_id: None,
            genre_ids: None,
            has_vacancies: Some(true),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSearchResult {
    pub id: Uuid,
    pub title: TitleVo,
    pub gm_display_name: String,
    pub description_snippet: Option<String>,
    pub system_name: String,
    pub genre_names: Vec<GameGenreVo>,
    pub max_players: Option<i32>,
    pub current_players_count: u32,
    pub contact_info_summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewTableData {
    pub gm_id: Uuid,
    pub title: TitleVo,
    pub description: Option<DescriptionVo>,
    pub system_id: i32,
    pub contact_info: ContactInfoVo,
    pub max_players: Option<u32>,
}
