use super::dtos::{CreateTableRequestCommand, UpdateTableRequestCommand};
use super::entity::TableRequest;
use crate::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRequestRepository: Send + Sync {
    async fn create(&self, request_data: &CreateTableRequestCommand) -> Result<String>;
    async fn update(&self, request_id: &Uuid, update_data: &UpdateTableRequestCommand) -> Result<()>;
    async fn delete(&self, request_id: &Uuid) -> Result<()>;
    async fn get(&self) -> Result<Vec<TableRequest>>;
    async fn find_by_id(&self, request_id: &Uuid) -> Result<Option<TableRequest>>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableRequest>>;
    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>>;
    async fn find_pending_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>>;
}
