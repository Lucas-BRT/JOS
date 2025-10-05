use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub description: String,
    pub player_slots: u32,
    pub game_system_id: Uuid,
    pub created_at: Date,
    pub updated_at: Date,
}
