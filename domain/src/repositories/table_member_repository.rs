use crate::{entities::*, repositories::Repository};
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableMemberRepository:
    Repository<
        TableMember,
        CreateTableMemberCommand,
        UpdateTableMemberCommand,
        GetTableMemberCommand,
        DeleteTableMemberCommand,
    > + Send
    + Sync
{
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableMember>>;
    async fn find_by_table_and_user(
        &self,
        table_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TableMember>>;
}
