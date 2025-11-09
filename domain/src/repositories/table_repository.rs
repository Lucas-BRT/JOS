use crate::entities::Table;
use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, command: &CreateTableCommand) -> Result<Table, Error>;
    async fn read(&self, command: &GetTableCommand) -> Result<Vec<Table>, Error>;
    async fn update(&self, command: &UpdateTableCommand) -> Result<Table, Error>;
    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Table>, Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Table>, Error>;
}
