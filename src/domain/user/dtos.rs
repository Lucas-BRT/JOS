use crate::domain::user::{entity::User, role::Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct MeResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

impl From<User> for MeResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            username: value.username,
            display_name: value.display_name,
            role: value.role,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserSummary {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
}

impl From<User> for UserSummary {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
        }
    }
}
