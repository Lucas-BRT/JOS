use crate::Result;
use crate::domain::entities::Table;
use crate::domain::entities::*;

#[async_trait::async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, command: CreateTableCommand) -> Result<Table>;
    async fn read(&self, command: GetTableCommand) -> Result<Vec<Table>>;
    async fn update(&self, command: UpdateTableCommand) -> Result<Table>;
    async fn delete(&self, command: DeleteTableCommand) -> Result<Table>;
    async fn search(&self, query: &str) -> Result<Vec<Table>>;
}
