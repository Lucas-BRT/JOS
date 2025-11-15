use crate::TableMemberService;
use domain::entities::*;
use domain::repositories::{TableMemberRepository, TableRepository, TableRequestRepository};
use shared::Result;
use shared::error::{DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableRequestService {
    table_request_repository: Arc<dyn TableRequestRepository>,
    table_repository: Arc<dyn TableRepository>,
    table_member_repository: Arc<dyn TableMemberRepository>,
    table_member_service: Arc<TableMemberService>,
}

impl TableRequestService {
    pub fn new(
        table_request_repository: Arc<dyn TableRequestRepository>,
        table_repository: Arc<dyn TableRepository>,
        table_member_repository: Arc<dyn TableMemberRepository>,
        table_member_service: Arc<TableMemberService>,
    ) -> Self {
        Self {
            table_request_repository,
            table_repository,
            table_member_repository,
            table_member_service,
        }
    }

    pub async fn create(&self, command: CreateTableRequestCommand) -> Result<TableRequest> {
        let table = self.table_repository.find_by_id(&command.table_id).await?;
        if let Some(table) = table {
            if table.gm_id == command.user_id {
                return Err(Error::Domain(DomainError::BusinessRuleViolation {
                    message: "Game master cannot request to join their own table".to_string(),
                }));
            }

            if table.status != domain::entities::TableStatus::Active {
                return Err(Error::Domain(DomainError::BusinessRuleViolation {
                    message: "Table is not accepting new players".to_string(),
                }));
            }
        } else {
            return Err(Error::Domain(DomainError::EntityNotFound {
                entity_type: "Table",
                entity_id: command.table_id.to_string(),
            }));
        }

        let existing_requests = self
            .table_request_repository
            .find_by_user_and_table(command.user_id, command.table_id)
            .await?;
        if existing_requests
            .iter()
            .any(|req| req.status == domain::entities::TableRequestStatus::Pending)
        {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "A pending request for this table already exists".to_string(),
            }));
        }

        let existing_member = self
            .table_member_repository
            .find_by_table_and_user(command.table_id, command.user_id)
            .await?;

        if existing_member.is_some() {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "User is already a member of this table".to_string(),
            }));
        }

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
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "TableRequest",
                entity_id: id.to_string(),
            })
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
