use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use axum::response::IntoResponse;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateTableRequestRequest {
    #[validate(length(max = 500))]
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SentRequestItem {
    pub id: Uuid,
    pub table_name: String,
    pub master: String,
    pub request_date: String,
    pub status: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ReceivedRequestItem {
    pub id: Uuid,
    pub player_name: String,
    pub table_name: String,
    pub request_date: String,
    pub experience: String,
    pub message: String,
    pub player_level: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableRequestResponse {
    pub id: Uuid,
    pub table_id: Uuid,
    pub player_id: Uuid,
    pub message: String,
    pub status: String,
    pub request_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AcceptRequestResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RejectRequestResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CancelRequestResponse {
    pub message: String,
}

// IntoResponse implementations
impl IntoResponse for TableRequestResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for AcceptRequestResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for RejectRequestResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for CancelRequestResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
