use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ITableService: Send + Sync {
    async fn create(&self, command: &CreateTableCommand) -> Result<Table, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Table>, Error>;
    async fn find_by_gm_id(&self, gm_id: Uuid) -> Result<Vec<Table>, Error>;
    async fn find_by_game_system_id(&self, game_system_id: Uuid) -> Result<Vec<Table>, Error>;
    async fn update(&self, command: &UpdateTableCommand) -> Result<Table, Error>;
    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table, Error>;
}
