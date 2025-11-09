use crate::entities::SessionIntent;
use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ISessionIntentService: Send + Sync {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent, Error>;
    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent, Error>;
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>, Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SessionIntent>, Error>;
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<SessionIntent>, Error>;
}
