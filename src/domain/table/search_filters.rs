use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableFilters {
    pub title: Option<String>,
    pub game_system_id: Option<Uuid>,
    pub available_slots: Option<u32>,
}
