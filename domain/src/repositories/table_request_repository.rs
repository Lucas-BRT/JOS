use crate::entities::TableRequest;
use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRequestRepository: Send + Sync {
    async fn create(&self, command: &CreateTableRequestCommand) -> Result<TableRequest, Error>;
    async fn read(&self, command: &GetTableRequestCommand) -> Result<Vec<TableRequest>, Error>;
    async fn update(&self, command: &UpdateTableRequestCommand) -> Result<TableRequest, Error>;
    async fn delete(&self, command: &DeleteTableRequestCommand) -> Result<TableRequest, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableRequest>, Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableRequest>, Error>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableRequest>, Error>;
    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>, Error>;
    async fn find_by_user_and_table(
        &self,
        user_id: Uuid,
        table_id: Uuid,
    ) -> Result<Vec<TableRequest>, Error>;
}
