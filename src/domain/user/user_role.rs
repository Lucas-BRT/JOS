use crate::infra::db::postgres::models::user::RowUserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[default]
    #[serde(rename = "user")]
    User,
}

impl From<RowUserRole> for UserRole {
    fn from(user_role: RowUserRole) -> Self {
        match user_role {
            RowUserRole::Admin => UserRole::Admin,
            RowUserRole::User => UserRole::User,
        }
    }
}
