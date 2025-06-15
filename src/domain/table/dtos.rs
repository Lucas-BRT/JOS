use crate::domain::{table::entity::PlayerExperience, utils::update::Update};
use uuid::Uuid;

pub struct CreateTableCommand {
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub player_slots: u32,
    pub bg_image_link: Option<String>,
    pub recommended_player_experience: Option<PlayerExperience>,
}

pub struct UpdateTableCommand {
    pub id: Uuid,
    pub title: Update<String>,
    pub description: Update<String>,
    pub game_system_id: Update<Uuid>,
    pub is_public: Update<bool>,
    pub player_slots: Update<u32>,
    pub recommended_player_experience: Update<Option<PlayerExperience>>,
    pub bg_image_link: Update<Option<String>>,
}
