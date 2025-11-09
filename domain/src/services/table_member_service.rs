pub use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ITableMemberService: Send + Sync {
    async fn create(&self, command: &CreateTableMemberCommand) -> Result<TableMember, Error>;
    async fn get(&self, command: &GetTableMemberCommand) -> Result<Vec<TableMember>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableMember>, Error>;
    async fn find_by_table_id(&self, id: Uuid) -> Result<Vec<TableMember>, Error>;
    async fn update(&self, command: &UpdateTableMemberCommand) -> Result<TableMember, Error>;
    async fn delete(&self, command: &DeleteTableMemberCommand) -> Result<TableMember, Error>;
}
