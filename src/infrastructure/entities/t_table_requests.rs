use super::enums::ETableRequestStatus;
use crate::domain::table_request::entity::TableRequest;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub user_id: Uuid,
    pub table_id: Uuid,
    pub message: Option<String>,
    pub status: ETableRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Model> for TableRequest {
    fn from(model: Model) -> Self {
        TableRequest {
            id: model.id,
            user_id: model.user_id,
            table_id: model.table_id,
            message: model.message,
            status: model.status.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
