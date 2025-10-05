use shared::Result;
use crate::entities::{
    CreateTableMemberCommand, DeleteTableMemberCommand, UpdateTableMemberCommand,
};
use crate::entities::{GetTableMemberCommand, TableMember};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableMemberRepository: Send + Sync {
    async fn create(&self, command: CreateTableMemberCommand) -> Result<TableMember>;
    async fn read(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>>;
    async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember>;
    async fn delete(&self, command: DeleteTableMemberCommand) -> Result<TableMember>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableMember>>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableMember>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<TableMember>>;
}
