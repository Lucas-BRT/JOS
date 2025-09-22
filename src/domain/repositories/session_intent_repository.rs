use crate::Result;
use crate::domain::entities::SessionIntent;
use crate::domain::entities::*;

#[async_trait::async_trait]
pub trait SessionIntentRepository: Send + Sync {
    async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent>;
    async fn read(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>>;
    async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent>;
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent>;
}
