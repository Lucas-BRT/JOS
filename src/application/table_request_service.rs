use crate::Result;
use crate::domain::entities::*;
use crate::domain::repositories::TableRequestRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableRequestService {
    table_request_repository: Arc<dyn TableRequestRepository>,
}

impl TableRequestService {
    pub fn new(table_request_repository: Arc<dyn TableRequestRepository>) -> Self {
        Self {
            table_request_repository,
        }
    }

    pub async fn create(&self, command: CreateTableRequestCommand) -> Result<TableRequest> {
        self.table_request_repository.create(command).await
    }

    pub async fn get(&self, command: GetTableRequestCommand) -> Result<Vec<TableRequest>> {
        self.table_request_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<TableRequest> {
        let command = GetTableRequestCommand {
            id: Some(*id),
            ..Default::default()
        };
        let table_requests = self.table_request_repository.read(command).await?;
        table_requests.into_iter().next().ok_or_else(|| {
            crate::Error::Domain(crate::domain::error::DomainError::TableRequest(
                crate::domain::error::TableRequestDomainError::TableRequestNotFound(id.to_string()),
            ))
        })
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableRequest>> {
        let command = GetTableRequestCommand {
            user_id: Some(*user_id),
            ..Default::default()
        };
        self.table_request_repository.read(command).await
    }

    pub async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableRequest>> {
        let command = GetTableRequestCommand {
            table_id: Some(*table_id),
            ..Default::default()
        };
        self.table_request_repository.read(command).await
    }

    pub async fn find_by_status(&self, status: &TableRequestStatus) -> Result<Vec<TableRequest>> {
        let command = GetTableRequestCommand {
            status: Some(*status),
            ..Default::default()
        };
        self.table_request_repository.read(command).await
    }

    pub async fn update(&self, command: UpdateTableRequestCommand) -> Result<TableRequest> {
        self.table_request_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteTableRequestCommand) -> Result<TableRequest> {
        self.table_request_repository.delete(command).await
    }
}
