use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use axum::response::IntoResponse;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateSessionRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: String,
    pub scheduled_at: DateTime<Utc>,
    pub table_id: Uuid,
    #[validate(range(min = 1, max = 20))]
    pub max_players: i32,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateSessionRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    #[validate(range(min = 1, max = 20))]
    pub max_players: Option<i32>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionPlayer {
    pub id: Uuid,
    pub name: String,
    pub character: String,
    pub avatar: String,
    pub status: String,
    pub is_current_user: bool,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionListItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub scheduled_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionDetails {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub scheduled_at: DateTime<Utc>,
    pub accepting_proposals: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub date: String,
    pub time: String,
    pub max_players: i32,
    pub master_id: Uuid,
    pub table_id: Uuid,
    pub players: Vec<SessionPlayer>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteSessionResponse {
    pub message: String,
}

// IntoResponse implementations
impl IntoResponse for SessionDetails {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for DeleteSessionResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
