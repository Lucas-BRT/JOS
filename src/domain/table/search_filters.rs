use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TableSearchFilters {
    pub query_text: Option<String>,
    pub system_id: Option<i32>,
    pub genre_ids: Option<Vec<i32>>,
    pub language: Option<String>,
    pub has_vacancies: Option<bool>,
}
