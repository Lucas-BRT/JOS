use super::dtos::{
    CreateTableRequestCommand, DeleteTableRequestCommand, GetTableRequestCommand,
    UpdateTableRequestCommand,
};
use super::entity::TableRequest;
use crate::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRequestRepository: Send + Sync {
    async fn create(&self, request_data: &CreateTableRequestCommand) -> Result<TableRequest>;
    async fn update(&self, update_data: &UpdateTableRequestCommand) -> Result<TableRequest>;
    async fn delete(&self, request_data: &DeleteTableRequestCommand) -> Result<TableRequest>;
    async fn get(&self, command: &GetTableRequestCommand) -> Result<Vec<TableRequest>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<TableRequest>;
}
