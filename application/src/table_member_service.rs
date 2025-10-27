use domain::entities::*;
use domain::repositories::TableMemberRepository;
use shared::Result;
use shared::error::{DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableMemberService {
    table_member_repository: Arc<dyn TableMemberRepository>,
}

impl TableMemberService {
    pub fn new(table_member_repository: Arc<dyn TableMemberRepository>) -> Self {
        Self {
            table_member_repository,
        }
    }

    pub async fn create(&self, command: CreateTableMemberCommand) -> Result<TableMember> {
        self.table_member_repository.create(command).await
    }

    pub async fn get(&self, command: GetTableMemberCommand) -> Result<Vec<TableMember>> {
        self.table_member_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<TableMember> {
        let command = GetTableMemberCommand {
            id: Some(*id),
            ..Default::default()
        };
        let table_members = self.table_member_repository.read(command).await?;
        table_members.into_iter().next().ok_or_else(|| {
            Error::Domain(DomainError::EntityNotFound {
                entity_type: "TableMember",
                entity_id: id.to_string(),
            })
        })
    }

    pub async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<TableMember>> {
        let command = GetTableMemberCommand {
            table_id: Some(*table_id),
            ..Default::default()
        };
        self.table_member_repository.read(command).await
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<TableMember>> {
        let command = GetTableMemberCommand {
            user_id: Some(*user_id),
            ..Default::default()
        };
        self.table_member_repository.read(command).await
    }

    pub async fn update(&self, command: UpdateTableMemberCommand) -> Result<TableMember> {
        self.table_member_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteTableMemberCommand) -> Result<TableMember> {
        self.table_member_repository.delete(command).await
    }
}
