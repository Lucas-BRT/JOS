use crate::domain::user::{entity::User, role::Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<User> for MeResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            role: value.role,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
