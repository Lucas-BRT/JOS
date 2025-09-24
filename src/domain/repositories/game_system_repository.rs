use crate::Result;
use crate::domain::entities::*;

#[async_trait::async_trait]
pub trait GameSystemRepository: Send + Sync {
    async fn create(&self, command: &mut CreateGameSystemCommand) -> Result<GameSystem>;
    async fn read(&self, command: &mut GetGameSystemCommand) -> Result<Vec<GameSystem>>;
    async fn update(&self, command: &mut UpdateGameSystemCommand) -> Result<GameSystem>;
    async fn delete(&self, command: &mut DeleteGameSystemCommand) -> Result<GameSystem>;
}
