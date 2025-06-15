use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub description: String,
    pub player_slots: i32,
    pub recommended_player_experience: Option<PlayerExperienceModel>,
    pub bg_image_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Type, Default)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "player_experience")]
pub enum PlayerExperienceModel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}
