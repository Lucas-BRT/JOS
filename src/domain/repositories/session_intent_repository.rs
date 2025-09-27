use crate::Result;
use crate::domain::entities::SessionIntent;
use crate::domain::entities::*;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionIntentRepository: Send + Sync {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent>;
    async fn read(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>>;
    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent>;
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SessionIntent>>;
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<SessionIntent>>;
}
