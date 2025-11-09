use domain::entities::*;
use domain::repositories::TableMemberRepository;
use domain::services::ITableMemberService;
use shared::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableMemberService<T: TableMemberRepository> {
    table_member_repository: T,
}

impl<T> TableMemberService<T>
where
    T: TableMemberRepository,
{
    pub fn new(table_member_repository: T) -> Self {
        Self {
            table_member_repository,
        }
    }
}

#[async_trait::async_trait]
impl<T> ITableMemberService for TableMemberService<T>
where
    T: TableMemberRepository,
{
    async fn create(&self, command: &CreateTableMemberCommand) -> Result<TableMember, Error> {
        self.table_member_repository.create(command).await
    }

    async fn get(&self, command: &GetTableMemberCommand) -> Result<Vec<TableMember>, Error> {
        self.table_member_repository.read(command).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TableMember>, Error> {
        self.table_member_repository.find_by_id(id).await
    }

    async fn find_by_table_id(&self, id: Uuid) -> Result<Vec<TableMember>, Error> {
        self.table_member_repository.find_by_table_id(id).await
    }

    async fn update(&self, command: &UpdateTableMemberCommand) -> Result<TableMember, Error> {
        self.table_member_repository.update(command).await
    }

    async fn delete(&self, command: &DeleteTableMemberCommand) -> Result<TableMember, Error> {
        self.table_member_repository.delete(command).await
    }
}
