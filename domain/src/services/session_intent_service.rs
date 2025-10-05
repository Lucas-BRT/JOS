use crate::entities::SessionIntent;
use crate::entities::*;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionIntentService: Send + Sync {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent>;
    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent>;
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SessionIntent>>;
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<SessionIntent>>;
}
