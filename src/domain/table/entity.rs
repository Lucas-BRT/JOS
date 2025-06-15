use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub description: String,
    pub player_slots: u32,
    pub recommended_player_experience: Option<PlayerExperience>,
    pub bg_image_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum PlayerExperience {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}
