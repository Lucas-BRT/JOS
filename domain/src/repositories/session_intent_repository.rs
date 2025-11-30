use crate::{entities::*, repositories::Repository};
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionIntentRepository:
    Repository<
        SessionIntent,
        CreateSessionIntentCommand,
        UpdateSessionIntentCommand,
        GetSessionIntentCommand,
        DeleteSessionIntentCommand,
    > + Send
    + Sync
{
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SessionIntent>>;
    async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<SessionIntent>>;
}
