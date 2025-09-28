use crate::domain::entities::TableMember;
use uuid::Uuid;
use crate::shared::Date;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct TableMemberModel {
    pub id: Uuid,
    pub table_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Date,
    pub updated_at: Date,
}

impl From<TableMemberModel> for TableMember {
    fn from(model: TableMemberModel) -> Self {
        TableMember {
            id: model.id,
            table_id: model.table_id,
            user_id: model.user_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
