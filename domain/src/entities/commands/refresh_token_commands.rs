use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRefreshTokenCommand {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateRefreshTokenCommand {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRefreshTokenCommand {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteRefreshTokenCommand {}
