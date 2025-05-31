use crate::infrastructure::persistance::postgres::models::user::AccessLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum UserAccessLevelVo {
    #[serde(rename = "admin")]
    Admin,
    #[default]
    #[serde(rename = "user")]
    User,
}

impl From<AccessLevel> for UserAccessLevelVo {
    fn from(access_level: AccessLevel) -> Self {
        match access_level {
            AccessLevel::Admin => UserAccessLevelVo::Admin,
            AccessLevel::User => UserAccessLevelVo::User,
        }
    }
}
