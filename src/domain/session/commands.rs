use crate::domain::{
    session::{filters::SessionFilters, entity::SessionStatus},
    utils::{pagination::Pagination, update::Update},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateSessionCommand {
    pub table_id: Uuid,
    pub name: String,
    pub description: String,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub status: SessionStatus,
    pub accepting_intents: bool,
}

impl CreateSessionCommand {
    pub fn new(
        table_id: Uuid, 
        name: String, 
        description: String, 
        scheduled_for: Option<DateTime<Utc>>,
        status: SessionStatus,
        accepting_intents: bool
    ) -> Self {
        Self {
            table_id,
            name,
            description,
            scheduled_for,
            status,
            accepting_intents,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetSessionCommand {
    pub filters: SessionFilters,
    pub pagination: Pagination,
}

impl GetSessionCommand {
    pub fn with_filters(self, filters: SessionFilters) -> Self {
        Self { filters, ..self }
    }

    pub fn with_pagination(self, pagination: Pagination) -> Self {
        Self { pagination, ..self }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateSessionCommand {
    pub id: Uuid,
    pub name: Update<String>,
    pub description: Update<String>,
    pub scheduled_for: Update<Option<DateTime<Utc>>>,
    pub status: Update<SessionStatus>,
    pub accepting_intents: Update<bool>,
}

impl UpdateSessionCommand {
    pub fn new(
        id: Uuid,
        name: Update<String>,
        description: Update<String>,
        scheduled_for: Update<Option<DateTime<Utc>>>,
        status: Update<SessionStatus>,
        accepting_intents: Update<bool>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            scheduled_for,
            status,
            accepting_intents,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteSessionCommand {
    pub id: Uuid,
}

impl DeleteSessionCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
