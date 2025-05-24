use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{user::user_min_info::UserMinInfo, utils::contact_info::ContactInfo};

use super::{
    description::Description, game_genre::GameGenre, system_min_info::SystemMinInfo, title::Title,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: Title,
    pub description: Option<Description>,
    pub system_id: u32,
    pub contact_info: ContactInfo,
    pub max_players: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
