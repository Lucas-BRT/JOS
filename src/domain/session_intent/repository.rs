use super::*;
use crate::Result;

#[async_trait::async_trait]
pub trait SessionIntentRepository: Send + Sync {
    async fn create(&self, session_intent: CreateSessionIntentCommand) -> Result<SessionIntent>;
    async fn update(&self, session_intent: UpdateSessionIntentCommand) -> Result<SessionIntent>;
    async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent>;
    async fn get(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>>;
}
