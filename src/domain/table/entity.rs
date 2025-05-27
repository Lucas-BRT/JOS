use super::vo::{DescriptionVo, TitleVo};
use crate::domain::utils::contact_info::ContactInfoVo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub gm_id: Uuid,
    pub title: TitleVo,
    pub description: Option<DescriptionVo>,
    pub system_id: u32,
    pub contact_info: ContactInfoVo,
    pub max_players: Option<u32>,
    pub created_at: DateTime<Utc>,
}
