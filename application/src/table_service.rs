use domain::entities::*;
use domain::repositories::TableRepository;
use domain::services::ITableService;
use shared::Error;
use shared::error::DomainError;
use uuid::Uuid;

#[derive(Clone)]
pub struct TableService<T>
where
    T: TableRepository,
{
    table_repository: T,
}

impl<T> TableService<T>
where
    T: TableRepository,
{
    pub fn new(table_repository: T) -> Self {
        Self { table_repository }
    }
}

#[async_trait::async_trait]
impl<T> ITableService for TableService<T>
where
    T: TableRepository,
{
    async fn find_by_gm_id(&self, gm_id: Uuid) -> Result<Vec<Table>, Error> {
        todo!()
    }

    async fn find_by_game_system_id(&self, game_system_id: Uuid) -> Result<Vec<Table>, Error> {
        todo!()
    }

    async fn create(&self, command: &CreateTableCommand) -> Result<Table, Error> {
        self.table_repository.create(command).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Table>, Error> {
        let table = self.table_repository.find_by_id(id).await?;

        Ok(table)
    }

    async fn update(&self, command: &UpdateTableCommand) -> Result<Table, Error> {
        self.table_repository.update(command).await
    }

    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table, Error> {
        let table = self.find_by_id(command.id).await?;

        match table {
            Some(table) => {
                if table.owner_id() != command.gm_id {
                    return Err(Error::Domain(DomainError::UserNotTableGameMaster));
                }

                self.table_repository.delete(command).await
            }
            None => Err(Error::Domain(DomainError::TableNotFound)),
        }
    }
}
