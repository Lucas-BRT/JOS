use crate::domain::user::{entity::User, role::Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MeResponse {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[schema(value_type = String)]
    pub role: Role,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
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
