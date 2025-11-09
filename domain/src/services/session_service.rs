pub use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ISessionService: Send + Sync {
    async fn create(&self, command: &CreateSessionCommand) -> Result<Session, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, Error>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>, Error>;
    async fn update(&self, command: &UpdateSessionCommand) -> Result<Session, Error>;
    async fn delete(&self, command: &DeleteSessionCommand) -> Result<Session, Error>;
}
