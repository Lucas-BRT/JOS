use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::TableMember;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableMemberModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
    pub status: String,
    pub character_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TableMemberModel> for TableMember {
    fn from(model: TableMemberModel) -> Self {
        TableMember {
            id: model.id,
            table_id: model.table_id,
            user_id: model.user_id,
            role: model.role,
            joined_at: model.joined_at,
            status: model.status,
            character_name: model.character_name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
