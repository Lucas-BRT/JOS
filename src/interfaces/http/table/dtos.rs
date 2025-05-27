use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTableDto {
    pub gm_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub system_id: u32,
    pub contact_info: String,
    pub max_players: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTableResponseDto {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTableDto {}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTableResponseDto {}
