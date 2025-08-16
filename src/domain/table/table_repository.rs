use super::commands::{CreateTableCommand, DeleteTableCommand, UpdateTableCommand};
use super::entity::Table;
use crate::Result;
use crate::domain::table::search_filters::TableFilters;
use crate::domain::utils::pagination::Pagination;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TableRepository: Send + Sync {
    async fn create(&self, command: &CreateTableCommand) -> Result<Table>;
    async fn update(&self, command: &UpdateTableCommand) -> Result<Table>;
    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table>;
    async fn get(&self, filters: &TableFilters, pagination: Pagination) -> Result<Vec<Table>>;
    async fn find_by_id(&self, table_id: &Uuid) -> Result<Table>;
    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>>;
}
