use crate::domain::table::{dtos::CreateTableCommand, entity::PlayerExperience};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

const MIN_TITLE_LENGTH: u64 = 8;
const MAX_TITLE_LENGTH: u64 = 60;

const MIN_DESCRIPTION_LENGTH: u64 = 50;
const MAX_DESCRIPTION_LENGTH: u64 = 1000;

const MIN_PLAYER_SLOTS: u32 = 1;
const MAX_PLAYER_SLOTS: u32 = 30;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTableDto {
    pub gm_id: Uuid,
    #[validate(length(min = MIN_TITLE_LENGTH, max = MAX_TITLE_LENGTH, message = "Title is empty"))]
    pub title: String,
    #[validate(length(min = MIN_DESCRIPTION_LENGTH, max = MAX_DESCRIPTION_LENGTH, message = "Description must be between 50 and 1000 characters"))]
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    #[validate(range(min = MIN_PLAYER_SLOTS, max = MAX_PLAYER_SLOTS, message = "Player slots must be between 1 and 30"))]
    pub player_slots: u32,
    pub bg_image_link: Option<String>,
    pub recommended_player_experience: Option<PlayerExperience>,
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
            bg_image_link: dto.bg_image_link,
            recommended_player_experience: dto.recommended_player_experience,
        }
    }
}
