use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::entity::AccessLevel as DomainAccessLevel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    Admin,
    User,
    Moderator,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::User
    }
}

impl From<DomainAccessLevel> for AccessLevel {
    fn from(access_level: DomainAccessLevel) -> Self {
        match access_level {
            DomainAccessLevel::Admin => AccessLevel::Admin,
            DomainAccessLevel::User => AccessLevel::User,
            DomainAccessLevel::Moderator => AccessLevel::Moderator,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Model {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub access_level: AccessLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
