use super::error::DescriptionValidationError;
use super::table::TableAggregate;
use crate::core::error::AppError;
use crate::domain::games::game_genre::GameGenre;
use crate::domain::table::{new_table::NewTableData, table::Table};
use crate::domain::utils::contact_info::ContactInfo;
use crate::domain::utils::pagination::Pagination;
use crate::domain::utils::type_wraper::TypeWrapped;
use crate::prelude::AppResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Title(String);

const MIN_DESCRIPTION_LENGTH: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 1000;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Description(String);

impl TypeWrapped for Description {
    type Raw = String;
    type Error = DescriptionValidationError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed = raw.trim();

        if trimmed.len() < MIN_DESCRIPTION_LENGTH {
            return Err(DescriptionValidationError::TooShort);
        }

        if trimmed.len() > MAX_DESCRIPTION_LENGTH {
            return Err(DescriptionValidationError::TooLong);
        }

        Ok(Description(trimmed.to_string()))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UpdateTableData {
    pub title: Option<Title>,
    pub description: Option<Option<Description>>,
    pub system_id: Option<u32>,
    pub contact_info: Option<ContactInfo>,
    pub max_players: Option<Option<u32>>,
    pub language: Option<String>,
}

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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TableSearchFilters {
    pub query_text: Option<String>,
    pub system_id: Option<i32>,
    pub genre_ids: Option<Vec<i32>>,
    pub language: Option<String>,
    pub has_vacancies: Option<bool>,
}

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
