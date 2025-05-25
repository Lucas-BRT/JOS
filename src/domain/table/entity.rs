use crate::domain::games::game_genre::GameGenre;
use crate::domain::games::min_info::SystemMinInfo;
use crate::domain::utils::type_wraper::TypeWrapped;
use crate::domain::{user::user_min_info::UserMinInfo, utils::contact_info::ContactInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::vo::{Description, Title};

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
