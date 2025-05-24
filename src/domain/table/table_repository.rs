use super::table::TableAggregate;
use super::table::Title;
use crate::domain::games::game_genre::GameGenre;
use crate::domain::table::{new_table::NewTableData, table::Table, update::UpdateTableData};
use crate::domain::utils::pagination::Pagination;
use crate::prelude::AppResult;
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

pub trait TableRepository {
    async fn create(&self, table_data: NewTableData, genre_ids: &[i32]) -> AppResult<Table>;

    async fn update(
        &self,
        table_id: Uuid,
        update_data: UpdateTableData,
        genre_ids_to_set: Option<&[i32]>,
    ) -> AppResult<Table>;

    async fn delete(&self, table_id: Uuid) -> AppResult<()>;

    async fn find_by_id(&self, table_id: Uuid) -> AppResult<Option<TableAggregate>>;

    async fn find_by_gm_id(&self, gm_id: Uuid, pagination: Pagination) -> AppResult<Vec<Table>>;

    async fn search_public_tables(
        &self,
        filters: TableSearchFilters,
        pagination: Pagination,
    ) -> AppResult<Vec<TableSearchResult>>;
}
