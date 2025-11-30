use crate::{entities::*, repositories::Repository};
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionCheckinRepository:
    Repository<
        SessionCheckin,
        CreateSessionCheckinCommand,
        UpdateSessionCheckinCommand,
        GetSessionCheckinCommand,
        DeleteSessionCheckinCommand,
    > + Send
    + Sync
{
    async fn find_by_session_intent_id(
        &self,
        session_intent_id: Uuid,
    ) -> Result<Vec<SessionCheckin>>;
    async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>>;
}
