use crate::domain::user::user_role::UserRole;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum RowUserRole {
    Admin,
    User,
}

impl From<UserRole> for RowUserRole {
    fn from(user_role: UserRole) -> Self {
        match user_role {
            UserRole::Admin => RowUserRole::Admin,
            UserRole::User => RowUserRole::User,
        }
    }
}

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserRow {
    pub id: sqlx::types::Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub user_role: RowUserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
