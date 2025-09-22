use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSessionDto {
    #[validate(length(min = 4, max = 100))]
    pub title: String,
    #[validate(length(min = 10, max = 1000))]
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    #[validate(range(min = 1, max = 20))]
    pub max_players: Option<u32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSessionDto {
    #[validate(length(min = 4, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 10, max = 1000))]
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    #[validate(range(min = 1, max = 20))]
    pub max_players: Option<u32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct JoinSessionDto {
    #[validate(length(min = 1, max = 100))]
    pub character_name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub table_id: Uuid,
    pub gm_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub max_players: Option<u32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JoinSessionResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeaveSessionResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConfirmSessionResponse {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeclineSessionResponse {
    pub message: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SessionFilters {
    pub table_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
