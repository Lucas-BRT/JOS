use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{description::Description, title::Title};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: Title,
    pub description: Option<Description>,
    pub system_id: u32,
    pub contact_info: String,
    pub max_players: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
