use crate::Result;
use crate::domain::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait GameSystemService: Send + Sync {
    async fn create(&self, command: &CreateGameSystemCommand) -> Result<GameSystem>;
    async fn get(&self, command: &GetGameSystemCommand) -> Result<Vec<GameSystem>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<GameSystem>;
    async fn find_by_name(&self, name: &str) -> Result<GameSystem>;
    async fn update(&self, command: &UpdateGameSystemCommand) -> Result<GameSystem>;
    async fn delete(&self, command: &DeleteGameSystemCommand) -> Result<GameSystem>;
}
