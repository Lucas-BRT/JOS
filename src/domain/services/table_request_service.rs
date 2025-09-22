use crate::Result;
use crate::domain::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRequestService: Send + Sync {
    async fn create(&self, command: &CreateTableRequestCommand) -> Result<TableRequest>;
    async fn get(&self, command: &GetTableRequestCommand) -> Result<Vec<TableRequest>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<TableRequest>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableRequest>>;
    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>>;
    async fn find_by_status(&self, status: &TableRequestStatus) -> Result<Vec<TableRequest>>;
    async fn update(&self, command: &UpdateTableRequestCommand) -> Result<TableRequest>;
    async fn delete(&self, command: &DeleteTableRequestCommand) -> Result<TableRequest>;
}
