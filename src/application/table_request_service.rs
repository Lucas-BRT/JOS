use crate::domain::table_request::dtos::{DeleteTableRequestCommand, TableRequestFilters};
use crate::domain::utils::pagination::Pagination;
use crate::Result;
use crate::domain::table::table_repository::TableRepository;
use crate::domain::table_request::{
    dtos::{CreateTableRequestCommand, UpdateTableRequestCommand},
    entity::TableRequest,
    table_request_repository::TableRequestRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableRequestService {
    table_request_repository: Arc<dyn TableRequestRepository>,
    table_service: Arc<dyn TableRepository>,
}

impl TableRequestService {
    pub fn new(
        table_request_repository: Arc<dyn TableRequestRepository>,
        table_service: Arc<dyn TableRepository>,
    ) -> Self {
        Self {
            table_request_repository,
            table_service,
        }
    }

    pub async fn create(&self, request_data: &mut CreateTableRequestCommand) -> Result<TableRequest> {
        self.table_request_repository.create(request_data).await
    }

    pub async fn update(
        &self,
        update_data: &UpdateTableRequestCommand,
    ) -> Result<()> {
        self.table_request_repository
            .update(update_data)
            .await
    }

    pub async fn delete(&self, request_data: &DeleteTableRequestCommand) -> Result<TableRequest> {
        self.table_request_repository.delete(request_data).await
    }

    pub async fn get(&self, filters: &TableRequestFilters, pagination: Pagination) -> Result<Vec<TableRequest>> {
        self.table_request_repository.get(filters, pagination).await
    }

    pub async fn get_requests_by_table_id(&self, table_id: &Uuid, gm_id: &Uuid) -> Result<Vec<TableRequest>> {
        let filters = TableRequestFilters {
            table_id: Some(*table_id),
            gm_id: Some(*gm_id),
            ..Default::default()
        };

        self.table_request_repository.get(&filters, Pagination::default()).await
    }
}
