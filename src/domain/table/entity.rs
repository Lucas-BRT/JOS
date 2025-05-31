use super::vo::{DescriptionVo, TitleVo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: TitleVo,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub description: DescriptionVo,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
