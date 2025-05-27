use crate::infrastructure::persistance::postgres::models::user::RowUserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum UserRoleVo {
    #[serde(rename = "admin")]
    Admin,
    #[default]
    #[serde(rename = "user")]
    User,
}

impl From<RowUserRole> for UserRoleVo {
    fn from(user_role: RowUserRole) -> Self {
        match user_role {
            RowUserRole::Admin => UserRoleVo::Admin,
            RowUserRole::User => UserRoleVo::User,
        }
    }
} 