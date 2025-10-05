use shared::Result;
use crate::entities::Table;
use crate::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, command: CreateTableCommand) -> Result<Table>;
    async fn read(&self, command: GetTableCommand) -> Result<Vec<Table>>;
    async fn update(&self, command: UpdateTableCommand) -> Result<Table>;
    async fn delete(&self, command: DeleteTableCommand) -> Result<Table>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Table>>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Table>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Table>>;
    async fn search(&self, query: &str) -> Result<Vec<Table>>;
}
