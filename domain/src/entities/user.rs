use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Date,
    pub updated_at: Date,
}
