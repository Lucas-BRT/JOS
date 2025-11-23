use axum::{
    Json,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain::entities::{IntentStatus, SessionCheckin, SessionIntent};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateSessionIntentRequest {
    pub session_id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateSessionIntentResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionIntentResponse {
    pub id: Uuid,
    pub player_id: Uuid,
    pub session_id: Uuid,
    pub status: IIntentStatus,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionIntentItem {
    pub id: Uuid,
    pub session_id: Uuid,
    pub status: IIntentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateSessionIntentRequest {
    pub status: Option<IIntentStatus>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteSessionIntentResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub enum IIntentStatus {
    #[default]
    Unsure,
    Confirmed,
    Declined,
}

impl From<IntentStatus> for IIntentStatus {
    fn from(value: IntentStatus) -> Self {
        match value {
            IntentStatus::Unsure => IIntentStatus::Unsure,
            IntentStatus::Confirmed => IIntentStatus::Confirmed,
            IntentStatus::Declined => IIntentStatus::Declined,
        }
    }
}

impl From<SessionIntent> for SessionIntentResponse {
    fn from(intent: SessionIntent) -> Self {
        Self {
            id: intent.id,
            player_id: intent.user_id,
            session_id: intent.session_id,
            status: intent.intent_status.into(),
        }
    }
}

impl From<SessionIntent> for SessionIntentItem {
    fn from(intent: SessionIntent) -> Self {
        Self {
            id: intent.id,
            session_id: intent.session_id,
            status: intent.intent_status.into(),
            created_at: intent.created_at,
        }
    }
}

// IntoResponse implementations
impl IntoResponse for SessionIntentResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for CreateSessionIntentResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for DeleteSessionIntentResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateSessionCheckinRequest {
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateSessionCheckinResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionCheckinResponse {
    pub id: Uuid,
    pub session_intent_id: Uuid,
    pub attendance: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateSessionCheckinRequest {
    pub attendance: Option<bool>,
    pub notes: Option<Option<String>>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteSessionCheckinResponse {
    pub message: String,
}

impl From<SessionCheckin> for SessionCheckinResponse {
    fn from(checkin: SessionCheckin) -> Self {
        Self {
            id: checkin.id,
            session_intent_id: checkin.session_intent_id,
            attendance: checkin.attendance,
            notes: checkin.notes,
            created_at: checkin.created_at,
            updated_at: checkin.updated_at,
        }
    }
}

// IntoResponse implementations
impl IntoResponse for SessionCheckinResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for CreateSessionCheckinResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for DeleteSessionCheckinResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
