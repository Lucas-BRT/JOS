use crate::entities::Table;
use crate::entities::*;
use crate::repositories::base::Repository;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRepository:
    Repository<Table, CreateTableCommand, UpdateTableCommand, GetTableCommand, DeleteTableCommand>
    + Send
    + Sync
{
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Table>>;
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Option<Table>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Table>>;
    async fn find_details_by_id(&self, table_id: Uuid) -> Result<Option<TableDetails>>;
}
