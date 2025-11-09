use domain::entities::*;
use domain::repositories::{SessionRepository, TableRepository};
use domain::services::ISessionService;
use shared::Error;
use shared::error::{ApplicationError, DomainError};
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionService<T, U>
where
    T: SessionRepository,
    U: TableRepository,
{
    session_repository: T,
    table_repository: U,
}

impl<T, U> SessionService<T, U>
where
    T: SessionRepository,
    U: TableRepository,
{
    pub fn new(session_repository: T, table_repository: U) -> Self {
        Self {
            session_repository,
            table_repository,
        }
    }
}

#[async_trait::async_trait]
impl<T, U> ISessionService for SessionService<T, U>
where
    T: SessionRepository,
    U: TableRepository,
{
    async fn create(&self, command: &CreateSessionCommand) -> Result<Session, Error> {
        let table = self.table_repository.find_by_id(command.table_id).await?;

        match table {
            Some(table) => {
                if table.owner_id() != command.gm_id {
                    return Err(Error::Domain(DomainError::UserNotTableGameMaster));
                }

                self.session_repository.create(command).await
            }
            None => return Err(Error::Domain(DomainError::TableNotFound)),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, Error> {
        self.session_repository.find_by_id(id).await
    }

    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>, Error> {
        self.session_repository.find_by_table_id(table_id).await
    }

    async fn update(&self, command: &UpdateSessionCommand) -> Result<Session, Error> {
        self.session_repository.update(command).await
    }

    async fn delete(&self, command: &DeleteSessionCommand) -> Result<Session, Error> {
        let table = match self.table_repository.find_by_id(command.table_id).await? {
            Some(t) => t,
            None => return Err(Error::Domain(DomainError::TableNotFound)),
        };

        if command.requester_id != table.owner_id() {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.session_repository.delete(command).await
    }
}
