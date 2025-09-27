use crate::Result;
use crate::domain::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableService: Send + Sync {
    async fn create(&self, command: &CreateTableCommand) -> Result<Table>;
    async fn get(&self, command: &GetTableCommand) -> Result<Vec<Table>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Table>;
    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>>;
    async fn find_by_game_system_id(&self, game_system_id: &Uuid) -> Result<Vec<Table>>;
    async fn update(&self, command: &UpdateTableCommand) -> Result<Table>;
    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table>;
}
