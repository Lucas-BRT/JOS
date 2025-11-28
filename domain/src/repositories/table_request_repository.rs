use crate::entities::TableRequest;
use crate::entities::*;
use crate::repositories::base::Repository;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRequestRepository:
    Repository<
        TableRequest,
        CreateTableRequestCommand,
        UpdateTableRequestCommand,
        GetTableRequestCommand,
        DeleteTableRequestCommand,
    > + Send
    + Sync
{
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableRequest>>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableRequest>>;
    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>>;
    async fn find_by_user_and_table(
        &self,
        user_id: Uuid,
        table_id: Uuid,
    ) -> Result<Vec<TableRequest>>;
}
