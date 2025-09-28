use crate::shared::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TableMember {
    pub id: Uuid,
    pub table_id: Uuid,
    pub user_id: Uuid,
    pub created_at: Date,
    pub updated_at: Date,
}
