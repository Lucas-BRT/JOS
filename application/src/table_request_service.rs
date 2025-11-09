use domain::entities::*;
use domain::repositories::{TableRepository, TableRequestRepository};
use domain::services::ITableRequestService;
use shared::Error;
use shared::error::DomainError;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableRequestService<T, U>
where
    T: TableRequestRepository,
    U: TableRepository,
{
    table_request_repository: T,
    table_repository: U,
}

#[async_trait::async_trait]
impl<T, U> ITableRequestService for TableRequestService<T, U>
where
    T: TableRequestRepository,
    U: TableRepository,
{
    async fn create(&self, command: &CreateTableRequestCommand) -> Result<TableRequest, Error> {
        let table = self.table_repository.find_by_id(command.table_id).await?;

        if table.is_none() {
            return Err(Error::Domain(DomainError::TableNotFound));
        }

        let existing_requests = self
            .table_request_repository
            .find_by_user_and_table(command.user_id, command.table_id)
            .await?;

        if existing_requests.iter().any(|req| req.pending()) {
            return Err(Error::Domain(DomainError::DuplicateTableRequest));
        }

        self.table_request_repository.create(command).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableRequest>, Error> {
        let table_requests = self.table_request_repository.find_by_id(id).await?;
        Ok(table_requests)
    }

    async fn find_by_user_id(&self, id: Uuid) -> Result<Vec<TableRequest>, Error> {
        self.table_request_repository.find_by_user_id(id).await
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<TableRequest>, Error> {
        self.table_request_repository
            .find_by_table_id(table_id)
            .await
    }

    async fn find_by_status(&self, status: TableRequestStatus) -> Result<Vec<TableRequest>, Error> {
        self.table_request_repository.find_by_status(status).await
    }

    async fn update(&self, command: &UpdateTableRequestCommand) -> Result<TableRequest, Error> {
        self.table_request_repository.update(command).await
    }

    async fn delete(&self, command: &DeleteTableRequestCommand) -> Result<TableRequest, Error> {
        self.table_request_repository.delete(command).await
    }
}

impl<T, U> TableRequestService<T, U>
where
    T: TableRequestRepository,
    U: TableRepository,
{
    pub fn new(table_request_repository: T, table_repository: U) -> Self {
        Self {
            table_request_repository,
            table_repository,
        }
    }
}
