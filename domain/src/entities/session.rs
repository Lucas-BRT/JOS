use serde::{Deserialize, Serialize};
use shared::prelude::Date;
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub table_id: Uuid,
    pub scheduled_for: Option<Date>,
    pub status: SessionStatus,
    pub created_at: Date,
    pub updated_at: Date,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ToSchema, Default)]
pub enum SessionStatus {
    #[default]
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionStatus::Scheduled => write!(f, "Scheduled"),
            SessionStatus::InProgress => write!(f, "In Progress"),
            SessionStatus::Completed => write!(f, "Completed"),
            SessionStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}
