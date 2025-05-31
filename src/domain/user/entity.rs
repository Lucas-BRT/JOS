use super::vo::{DisplayNameVo, EmailVo, UserAccessLevelVo, UsernameVo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: UsernameVo,
    pub display_name: DisplayNameVo,
    pub email: EmailVo,
    pub access_level: UserAccessLevelVo,
}
