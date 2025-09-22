use crate::Result;
use crate::domain::entities::{
    CreateTableMemberCommand, DeleteTableMemberCommand, UpdateTableMemberCommand,
};
use crate::domain::entities::{GetTableMemberCommand, TableMember};

#[async_trait::async_trait]
pub trait TableMemberRepository: Send + Sync {
    async fn create(&self, command: CreateTableMemberCommand) -> Result<TableMember>;
    async fn read(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>>;
    async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember>;
    async fn delete(&self, command: DeleteTableMemberCommand) -> Result<TableMember>;
}
