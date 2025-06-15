use super::dtos::{CreateTableCommand, UpdateTableCommand};
use super::entity::Table;
use crate::Result;
use crate::domain::utils::pagination::Pagination;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, table_data: &CreateTableCommand) -> Result<String>;
    async fn update(&self, update_data: &UpdateTableCommand) -> Result<()>;
    async fn delete(&self, table_id: &Uuid) -> Result<()>;
    async fn find_by_id(&self, table_id: &Uuid) -> Result<Option<Table>>;
    async fn find_by_gm_id(&self, gm_id: &Uuid, pagination: &Pagination) -> Result<Vec<Table>>;
}
