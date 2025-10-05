use shared::Result;
use crate::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait GameSystemRepository: Send + Sync {
    async fn create(&self, command: &mut CreateGameSystemCommand) -> Result<GameSystem>;
    async fn read(&self, command: &mut GetGameSystemCommand) -> Result<Vec<GameSystem>>;
    async fn update(&self, command: &mut UpdateGameSystemCommand) -> Result<GameSystem>;
    async fn delete(&self, command: &mut DeleteGameSystemCommand) -> Result<GameSystem>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>>;
}
