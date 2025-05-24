use super::{contact_info::ContactInfo, description::Description, title::Title};
use crate::domain::{
    game_genre::GameGenre, system_min_info::SystemMinInfo, user_min_info::UserMinInfo,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableAggregate {
    pub id: Uuid,
    pub gm_info: UserMinInfo,
    pub title: Title,
    pub description: Option<Description>,
    pub system_info: SystemMinInfo,
    pub genres: Vec<GameGenre>,
    pub contact_info: ContactInfo,
    pub max_players: Option<u32>,
    pub language: String,
    pub current_players: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
