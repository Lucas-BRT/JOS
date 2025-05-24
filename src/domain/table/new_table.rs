use super::{contact_info::ContactInfo, description::Description, title::Title};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewTableData {
    pub gm_id: Uuid,
    pub title: Title,
    pub description: Option<Description>,
    pub system_id: i32,
    pub contact_info: ContactInfo,
    pub max_players: Option<u32>,
}
