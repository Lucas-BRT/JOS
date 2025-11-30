use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGameSystemCommand {
    pub id: Uuid,
    pub name: String,
}

impl CreateGameSystemCommand {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGameSystemCommand {
    pub id: Uuid,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGameSystemCommand {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteGameSystemCommand {
    pub id: Uuid,
}
