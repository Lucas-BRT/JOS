use crate::domain::table::entity::Visibility;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableFilters {
    pub id: Option<Uuid>,
    pub gm_id: Option<Uuid>,
    pub title: Option<String>,
    pub visibility: Option<Visibility>,
    pub description: Option<String>,
    pub game_system_id: Option<Uuid>,
    pub player_slots: Option<u32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TableFilters {
    pub fn new() -> Self {
        Self {
            id: None,
            gm_id: None,
            title: None,
            visibility: None,
            description: None,
            game_system_id: None,
            player_slots: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn with_id(self, id: Uuid) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn with_gm_id(self, gm_id: Uuid) -> Self {
        Self {
            gm_id: Some(gm_id),
            ..self
        }
    }

    pub fn with_title(self, title: String) -> Self {
        Self {
            title: Some(title),
            ..self
        }
    }

    pub fn with_visibility(self, visibility: Visibility) -> Self {
        Self {
            visibility: Some(visibility),
            ..self
        }
    }

    pub fn with_description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn with_game_system_id(self, game_system_id: Uuid) -> Self {
        Self {
            game_system_id: Some(game_system_id),
            ..self
        }
    }

    pub fn with_player_slots(self, player_slots: u32) -> Self {
        Self {
            player_slots: Some(player_slots),
            ..self
        }
    }

    pub fn with_created_at(self, created_at: DateTime<Utc>) -> Self {
        Self {
            created_at: Some(created_at),
            ..self
        }
    }

    pub fn with_updated_at(self, updated_at: DateTime<Utc>) -> Self {
        Self {
            updated_at: Some(updated_at),
            ..self
        }
    }
}
