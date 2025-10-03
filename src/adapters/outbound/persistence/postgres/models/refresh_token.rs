use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RefreshTokenRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<RefreshTokenRow> for crate::domain::entities::RefreshToken {
    fn from(r: RefreshTokenRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            token: r.token,
            expires_at: r.expires_at,
            created_at: r.created_at,
        }
    }
}
