use super::*;
use crate::domain::utils::{pagination::Pagination, update::Update};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct CreateSessionIntentCommand {
    pub player_id: Uuid,
    pub session_id: Uuid,
    pub status: IntentStatus,
}

impl CreateSessionIntentCommand {
    pub fn new(player_id: Uuid, session_id: Uuid, status: IntentStatus) -> Self {
        Self {
            player_id,
            session_id,
            status,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateSessionIntentCommand {
    pub id: Uuid,
    pub status: Update<IntentStatus>,
}

impl UpdateSessionIntentCommand {
    pub fn new(id: Uuid, status: Update<IntentStatus>) -> Self {
        Self { id, status }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteSessionIntentCommand {
    pub id: Uuid,
}

impl DeleteSessionIntentCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetSessionIntentCommand {
    pub filters: SessionIntentFilter,
    pub pagination: Pagination,
}

impl GetSessionIntentCommand {
    pub fn new(filters: SessionIntentFilter, pagination: Pagination) -> Self {
        Self {
            filters,
            pagination,
        }
    }

    pub fn with_filters(mut self, filters: SessionIntentFilter) -> Self {
        self.filters = filters;
        self
    }

    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination.page_size = limit;
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.pagination.page = offset;
        self
    }
}
