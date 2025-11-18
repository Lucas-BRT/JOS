use chrono::{DateTime, Utc};
use domain::entities::TableMember;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ToSchema)]
pub struct TableMemberResponse {
    joined_at: DateTime<Utc>,
    user_id: Uuid,
    table_id: Uuid,
}

impl From<TableMember> for TableMemberResponse {
    fn from(value: TableMember) -> Self {
        Self {
            joined_at: value.created_at,
            user_id: value.user_id,
            table_id: value.table_id,
        }
    }
}
