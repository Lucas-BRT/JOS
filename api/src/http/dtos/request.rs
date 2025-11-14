use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use domain::entities::{TableRequest, TableRequestStatus};
use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateTableRequestRequest {
    #[validate(length(max = 500))]
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SentRequestItem {
    pub id: Uuid,
    pub request_date: DateTime<Utc>,
    pub status: ITableRequestStatus,
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub enum ITableRequestStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}

impl From<TableRequestStatus> for ITableRequestStatus {
    fn from(value: TableRequestStatus) -> Self {
        match value {
            TableRequestStatus::Pending => ITableRequestStatus::Pending,
            TableRequestStatus::Approved => ITableRequestStatus::Approved,
            TableRequestStatus::Rejected => ITableRequestStatus::Rejected,
        }
    }
}

impl From<TableRequest> for SentRequestItem {
    fn from(request: TableRequest) -> Self {
        SentRequestItem {
            id: request.id,
            request_date: request.created_at,
            status: request.status.into(),
            message: request.message,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ReceivedRequestItem {
    pub id: Uuid,
    pub player_id: Uuid,
    pub table_id: Uuid,
    pub request_date: Date,
    pub message: Option<String>,
}

impl From<TableRequest> for ReceivedRequestItem {
    fn from(request: TableRequest) -> Self {
        ReceivedRequestItem {
            id: request.id,
            player_id: request.user_id,
            table_id: request.table_id,
            request_date: request.created_at,
            message: request.message,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct TableRequestResponse {
    pub id: Uuid,
    pub table_id: Uuid,
    pub player_id: Uuid,
    pub message: Option<String>,
    pub status: ITableRequestStatus,
    pub request_date: DateTime<Utc>,
}

impl From<TableRequest> for TableRequestResponse {
    fn from(request: TableRequest) -> Self {
        TableRequestResponse {
            id: request.id,
            table_id: request.table_id,
            player_id: request.user_id,
            message: request.message,
            status: request.status.into(),
            request_date: request.created_at,
        }
    }
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
