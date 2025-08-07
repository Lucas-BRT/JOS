use crate::Result;
use crate::domain::table_request::{
    dtos::{CreateTableRequestCommand, UpdateTableRequestCommand},
    entity::TableRequest,
    table_request_repository::TableRequestRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableRequestService {
    repository: Arc<dyn TableRequestRepository>,
}

impl TableRequestService {
    pub fn new(repository: Arc<dyn TableRequestRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request_data: &CreateTableRequestCommand) -> Result<String> {
        self.repository.create(request_data).await
    }

    pub async fn update(&self, request_id: &Uuid, update_data: &UpdateTableRequestCommand) -> Result<()> {
        self.repository.update(request_id, update_data).await
    }

    pub async fn delete(&self, request_id: &Uuid) -> Result<()> {
        self.repository.delete(request_id).await
    }

    pub async fn get(&self) -> Result<Vec<TableRequest>> {
        self.repository.get().await
    }

    pub async fn find_by_id(&self, request_id: &Uuid) -> Result<Option<TableRequest>> {
        self.repository.find_by_id(request_id).await
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableRequest>> {
        self.repository.find_by_user_id(user_id).await
    }

    pub async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>> {
        self.repository.find_by_table_id(table_id).await
    }

    pub async fn find_pending_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>> {
        self.repository.find_pending_by_table_id(table_id).await
    }
}
