use crate::Result;
use crate::domain::entities::{
    CreateGameSystemCommand, DeleteGameSystemCommand, GameSystem, GetGameSystemCommand,
    UpdateGameSystemCommand,
};

#[async_trait::async_trait]
pub trait GameSystemRepository: Send + Sync {
    async fn create(&self, command: CreateGameSystemCommand) -> Result<GameSystem>;
    async fn read(&self, command: GetGameSystemCommand) -> Result<Vec<GameSystem>>;
    async fn update(&self, command: UpdateGameSystemCommand) -> Result<GameSystem>;
    async fn delete(&self, command: DeleteGameSystemCommand) -> Result<GameSystem>;
}
