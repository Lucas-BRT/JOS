use crate::entities::GameSystem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::error::DomainError;
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ToSchema, Default)]
pub enum TableStatus {
    #[default]
    Active,
    Finished,
    Cancelled,
}

impl fmt::Display for TableStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TableStatus::Active => write!(f, "Active"),
            TableStatus::Finished => write!(f, "Finished"),
            TableStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl FromStr for TableStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(TableStatus::Active),
            "Finished" => Ok(TableStatus::Finished),
            "Cancelled" => Ok(TableStatus::Cancelled),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Table<'a> {
    id: Uuid,
    gm_id: Uuid,
    title: &'a str,
    description: &'a str,
    player_slots: u32,
    status: TableStatus,
    game_system: GameSystem,
    created_at: DateTime<Utc>,
}

impl<'a> Table<'a> {
    pub fn new(
        gm_id: Uuid,
        title: &'a str,
        description: &'a str,
        game_system: GameSystem,
        player_slots: u32,
    ) -> Result<Self, DomainError> {
        let id = Uuid::new_v7(Timestamp::now(NoContext));

        let title_len = title.len();
        if title_len == 0 {
            return Err(DomainError::EmptyTitle);
        }

        let status = TableStatus::Active;
        let created_at = Utc::now();

        Ok(Table {
            id,
            gm_id,
            title,
            description,
            player_slots,
            status,
            game_system,
            created_at,
        })
    }

    pub fn owner_id(&self) -> Uuid {
        self.gm_id
    }

    pub fn current_status(&self) -> TableStatus {
        self.status
    }

    pub fn active(&self) -> bool {
        self.status == TableStatus::Active
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::error::DomainError;

    #[test]
    fn new_table_fails_with_empty_title() {
        let game_system =
            GameSystem::new("Dungeons and Dragons").expect("failed to create GameSystem");
        let result = Table::new(Uuid::new_v4(), "", "", game_system, 4);

        assert!(matches!(result, Err(DomainError::EmptyTitle)));
    }
}
