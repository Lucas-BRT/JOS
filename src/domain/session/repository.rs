use crate::Result;
use crate::domain::session::entity::Session;
use uuid::Uuid;

use crate::domain::session::{
    CreateSessionCommand, DeleteSessionCommand, GetSessionCommand, UpdateSessionCommand,
};

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &CreateSessionCommand) -> Result<Session>;
    async fn get(&self, command: &GetSessionCommand) -> Result<Vec<Session>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Session>>;
    async fn find_by_table_id(&self, table_id: &Uuid) -> Result<Vec<Session>>;
    async fn update(&self, command: &UpdateSessionCommand) -> Result<Session>;
    async fn delete(&self, command: &DeleteSessionCommand) -> Result<Session>;
}
