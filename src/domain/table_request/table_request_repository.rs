use super::dtos::{
    CreateTableRequestCommand, DeleteTableRequestCommand, TableRequestFilters,
    UpdateTableRequestCommand,
};
use super::entity::TableRequest;
use crate::Result;
use crate::domain::utils::pagination::Pagination;

#[async_trait::async_trait]
pub trait TableRequestRepository: Send + Sync {
    async fn create(&self, request_data: &mut CreateTableRequestCommand) -> Result<TableRequest>;
    async fn update(&self, update_data: &UpdateTableRequestCommand) -> Result<()>;
    async fn delete(&self, request_data: &DeleteTableRequestCommand) -> Result<TableRequest>;
    async fn get(
        &self,
        filters: &TableRequestFilters,
        pagination: Pagination,
    ) -> Result<Vec<TableRequest>>;
    async fn find(&self, filters: &TableRequestFilters) -> Result<TableRequest>;
}
