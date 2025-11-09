use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait GameSystemRepository: Send + Sync {
    async fn create(&self, command: &CreateGameSystemCommand) -> Result<GameSystem, Error>;
    async fn read(&self, command: &GetGameSystemCommand) -> Result<Vec<GameSystem>, Error>;
    async fn update(&self, command: &UpdateGameSystemCommand) -> Result<GameSystem, Error>;
    async fn delete(&self, command: &DeleteGameSystemCommand) -> Result<GameSystem, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>, Error>;
    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>, Error>;
}
