use crate::entities::{MAX_GAMESYSTEM_NAME_LENGTH, MIN_GAMESYSTEM_NAME_LENGTH};
use serde::{Deserialize, Serialize};
use shared::{Error, error::DomainError};
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSystem {
    pub id: Uuid,
    pub name: String,
}

impl GameSystem {
    pub fn new(name: &str) -> Result<Self, DomainError> {
        let id = Uuid::new_v7(Timestamp::now(NoContext));
        if name.trim().is_empty() {
            return Err(DomainError::EmptyTitle);
        }

        let len = name.len();

        if len > MAX_GAMESYSTEM_NAME_LENGTH {
            return Err(DomainError::NameTooLong);
        }

        if len < MIN_GAMESYSTEM_NAME_LENGTH {
            return Err(DomainError::NameTooShort);
        }

        Ok(Self {
            id,
            name: name.to_string(),
        })
    }
}

impl Default for GameSystem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Default Game System".into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameSystemBuilder {
    id: Option<Uuid>,
    name: Option<String>,
}

impl GameSystemBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<GameSystem, Error> {
        let id = self
            .id
            .ok_or(Error::Domain(DomainError::MissingField("id".into())))?;
        let name = self
            .name
            .ok_or(Error::Domain(DomainError::MissingField("name".into())))?;

        Ok(GameSystem {
            id,
            name: name.to_string(),
        })
    }
}
