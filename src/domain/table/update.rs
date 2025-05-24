use serde::{Deserialize, Serialize};

use crate::domain::utils::contact_info::ContactInfo;

use super::{description::Description, title::Title};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UpdateTableData {
    pub title: Option<Title>,
    pub description: Option<Option<Description>>,
    pub system_id: Option<u32>,
    pub contact_info: Option<ContactInfo>,
    pub max_players: Option<Option<u32>>,
    pub language: Option<String>,
}
