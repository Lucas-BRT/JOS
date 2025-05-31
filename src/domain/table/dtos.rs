use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{core::error::AppError, domain::{games::game_genre::GameGenreVo, utils::{contact_info::ContactInfoTypeVo, type_wraper::TypeWrapped}}, interfaces::http::table::dtos::CreateTableDto};

use super::{error::TableDomainError, vo::{DescriptionVo, TitleVo}};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UpdateTableData {
    pub title: Option<TitleVo>,
    pub description: Option<Option<DescriptionVo>>,
    pub system_id: Option<u32>,
    pub contact_info: Option<ContactInfoTypeVo>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSearchFilters {
    pub query_text: Option<String>,
    pub system_id: Option<i32>,
    pub genre_ids: Option<Vec<i32>>,
    pub gm_id: Option<Uuid>,
    pub min_players: Option<i32>,
    pub max_players: Option<i32>,
    pub has_vacancies: Option<bool>,
}

impl Default for TableSearchFilters {
    fn default() -> Self {
        Self {
            query_text: None,
            system_id: None,
            genre_ids: None,
            gm_id: None,
            min_players: None,
            max_players: None,
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
    pub system_id: Uuid,
    pub is_public: bool,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

impl TryFrom<&CreateTableDto> for NewTableData {
    type Error = TableDomainError;

    fn try_from(value: &CreateTableDto) -> Result<Self, Self::Error> {
        let title = TitleVo::parse(value.title.clone())?;

        let description = if value.description.is_some() {
            Some(DescriptionVo::parse(value.description.clone().unwrap())?)
        } else {
            None
        };


        let bg_image_link = if value.bg_image_link.is_some() {
            Some(value.bg_image_link.clone().unwrap())
        } else {
            None
        };

        Ok(Self {
            gm_id: value.gm_id,
            title,
            description,
            system_id: value.system_id,
            is_public: true,
            player_slots: value.player_slots,
            occupied_slots: value.occupied_slots,
            bg_image_link,
        })
    }
}