use crate::Result;
use crate::domain::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableMemberService: Send + Sync {
    async fn create(&self, command: &CreateTableMemberCommand) -> Result<TableMember>;
    async fn get(&self, command: &GetTableMemberCommand) -> Result<Vec<TableMember>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<TableMember>;
    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableMember>>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableMember>>;
    async fn update(&self, command: &UpdateTableMemberCommand) -> Result<TableMember>;
    async fn delete(&self, command: &DeleteTableMemberCommand) -> Result<TableMember>;
}
