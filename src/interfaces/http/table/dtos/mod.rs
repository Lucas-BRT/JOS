use serde::Deserialize;
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
    pub description: Option<String>,
    pub system_id: Uuid,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}
