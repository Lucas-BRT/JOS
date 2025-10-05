use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSystem {
    pub id: Uuid,
    pub name: String,
    pub created_at: Date,
    pub updated_at: Date,
}
