use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ITableRequestService: Send + Sync {
    async fn create(&self, command: &CreateTableRequestCommand) -> Result<TableRequest, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableRequest>, Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableRequest>, Error>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableRequest>, Error>;
    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>, Error>;
    async fn update(&self, command: &UpdateTableRequestCommand) -> Result<TableRequest, Error>;
    async fn delete(&self, command: &DeleteTableRequestCommand) -> Result<TableRequest, Error>;
}
