use super::enums::ETableVisibility;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: String,
    pub visibility: ETableVisibility,
    pub description: String,
    pub game_system_id: Uuid,
    pub max_players: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
