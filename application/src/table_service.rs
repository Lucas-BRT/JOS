use domain::entities::*;
use domain::repositories::{TableRepository, TableRequestRepository};
use shared::Result;
use shared::error::DomainError;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableService {
    table_repository: Arc<dyn TableRepository>,
    table_request_repository: Arc<dyn TableRequestRepository>,
}

impl TableService {
    pub fn new(
        table_repository: Arc<dyn TableRepository>,
        table_request_repository: Arc<dyn TableRequestRepository>,
    ) -> Self {
        Self {
            table_repository,
            table_request_repository,
        }
    }

    pub async fn find_by_id(&self, table_id: Uuid) -> Result<Table> {
        let table = self.table_repository.find_by_id(table_id).await?;
        table.ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: table_id.to_string(),
            })
        })
    }

    pub async fn find_table_by_user_id(&self, user_id: Uuid) -> Result<Vec<Table>> {
        self.table_repository.find_by_user_id(user_id).await
    }

    pub async fn find_table_by_session_id(&self, session_id: Uuid) -> Result<Option<Table>> {
        self.table_repository.find_by_session_id(session_id).await
    }

    pub async fn get_all_tables(&self) -> Result<Vec<Table>> {
        self.table_repository.read(GetTableCommand::default()).await
    }

    pub async fn get_table_details(
        &self,
        table_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TableDetails>> {
        let table_details = self.table_repository.find_details_by_id(table_id).await?;

        Ok(table_details)
    }

    pub async fn update_table(&self, command: UpdateTableCommand) -> Result<Table> {
        self.table_repository.update(command).await
    }

    pub async fn get_table_requests(&self, table_id: Uuid) -> Result<Vec<TableRequest>> {
        let requests = self
            .table_request_repository
            .find_by_table_id(table_id)
            .await?;

        Ok(requests)
    }

    pub async fn create_table(&self, command: CreateTableCommand) -> Result<Table> {
        self.table_repository.create(command).await
    }

    pub async fn delete_table(&self, table_id: Uuid, user_id: Uuid) -> Result<()> {
        let command = DeleteTableCommand {
            id: table_id,
            gm_id: user_id,
        };

        self.table_repository.delete(command).await?;
        Ok(())
    }
}
