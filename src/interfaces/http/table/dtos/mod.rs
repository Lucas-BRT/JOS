use crate::domain::table::{dtos::CreateTableCommand, entity::Table};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTableDto {
    pub gm_id: Uuid,
    #[validate(length(min = 8, max = 60, message = "Title is empty"))]
    pub title: String,
    #[validate(length(
        min = 50,
        max = 1000,
        message = "Description must be between 50 and 1000 characters"
    ))]
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

impl From<CreateTableDto> for CreateTableCommand {
    fn from(dto: CreateTableDto) -> Self {
        CreateTableCommand {
            gm_id: dto.gm_id,
            title: dto.title,
            description: dto.description,
            game_system_id: dto.game_system_id,
            is_public: dto.is_public,
            player_slots: dto.player_slots,
            occupied_slots: dto.occupied_slots,
            bg_image_link: dto.bg_image_link,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AvaliableTableResponse {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

impl From<Table> for AvaliableTableResponse {
    fn from(table: Table) -> Self {
        Self {
            gm_id: table.gm_id,
            title: table.title,
            description: table.description,
            game_system_id: table.game_system_id,
            is_public: table.is_public,
            player_slots: table.player_slots,
            occupied_slots: table.occupied_slots,
            bg_image_link: table.bg_image_link,
        }
    }
}
