use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use domain::entities::{Session, SessionStatus};
use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub enum ISessionStatus {
    #[default]
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl From<SessionStatus> for ISessionStatus {
    fn from(value: SessionStatus) -> Self {
        match value {
            SessionStatus::Scheduled => ISessionStatus::Scheduled,
            SessionStatus::InProgress => ISessionStatus::InProgress,
            SessionStatus::Completed => ISessionStatus::Completed,
            SessionStatus::Cancelled => ISessionStatus::Cancelled,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateSessionRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: String,
    pub scheduled_for: Option<Date>,
    pub status: Option<SessionStatus>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateSessionResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateSessionRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub scheduled_for: Option<Date>,
    #[validate(range(min = 1, max = 20))]
    pub max_players: Option<i32>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateSessionResponse {
    pub id: Uuid,
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
pub struct GetSessionsResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub scheduled_for: Option<Date>,
}

impl From<&Session> for GetSessionsResponse {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id,
            title: session.title.clone(),
            description: session.description.clone(),
            scheduled_for: session.scheduled_for,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionListItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub scheduled_for: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionDetails {
    pub id: Uuid,
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
