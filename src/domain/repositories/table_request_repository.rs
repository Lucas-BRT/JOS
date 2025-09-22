use crate::Result;
use crate::domain::entities::TableRequest;
use crate::domain::entities::*;

#[async_trait::async_trait]
pub trait TableRequestRepository: Send + Sync {
    async fn create(&self, command: CreateTableRequestCommand) -> Result<TableRequest>;
    async fn read(&self, command: GetTableRequestCommand) -> Result<Vec<TableRequest>>;
    async fn update(&self, command: UpdateTableRequestCommand) -> Result<TableRequest>;
    async fn delete(&self, command: DeleteTableRequestCommand) -> Result<TableRequest>;
}
