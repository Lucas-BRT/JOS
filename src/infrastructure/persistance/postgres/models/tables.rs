use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    pub description: String,
    pub player_slots: i32,
    pub occupied_slots: i32,
    pub bg_image_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
