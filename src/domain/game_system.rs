use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSystem {
    pub id: Uuid,
    pub name: String,
}

#[async_trait::async_trait]
pub trait GameSystemRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<GameSystem>;
    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>>;
    async fn get_all(&self) -> Result<Vec<GameSystem>>;
}
