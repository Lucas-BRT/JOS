use crate::domain::table_request::{dtos::CreateTableRequestCommand, entity::TableRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateTableRequestDto {
    pub table_id: Uuid,
    #[validate(length(max = 500, message = "Message must be less than 500 characters"))]
    pub message: Option<String>,
}

impl CreateTableRequestCommand {
    pub fn from_dto(dto: CreateTableRequestDto, user_id: Uuid) -> Self {
        CreateTableRequestCommand {
            user_id,
            table_id: dto.table_id,
            message: dto.message,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateTableRequestDto {
    #[validate(length(min = 1, message = "Status cannot be empty"))]
    pub status: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TableRequestResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<&TableRequest> for TableRequestResponse {
    fn from(request: &TableRequest) -> Self {
        Self {
            id: request.id,
            user_id: request.user_id,
            table_id: request.table_id,
            message: request.message.clone(),
            status: request.status.clone().into(),
            created_at: request.created_at,
            updated_at: request.updated_at,
        }
    }
}
