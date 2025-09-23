use crate::domain::entities::TableMember;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableMemberModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TableMemberModel> for TableMember {
    fn from(model: TableMemberModel) -> Self {
        TableMember {
            id: model.id,
            table_id: model.table_id,
            user_id: model.user_id,
            joined_at: model.joined_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
