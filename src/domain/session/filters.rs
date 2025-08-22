use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct SessionFilters {
    pub id: Option<Uuid>,
    pub table_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub accepting_intents: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SessionFilters {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_table_id(mut self, table_id: Uuid) -> Self {
        self.table_id = Some(table_id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_accepting_intents(mut self, accepting_intents: bool) -> Self {
        self.accepting_intents = Some(accepting_intents);
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
}
