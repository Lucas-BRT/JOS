use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserFilters {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub nickname: Option<String>,
}
