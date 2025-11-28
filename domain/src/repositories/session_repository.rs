use crate::entities::session_checkin::{SessionFinalizationData, SessionFinalizationResult};
pub use crate::entities::*;
use crate::repositories::base::Repository;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionRepository:
    Repository<
        Session,
        CreateSessionCommand,
        UpdateSessionCommand,
        GetSessionCommand,
        DeleteSessionCommand,
    > + Send
    + Sync
{
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>>;
    async fn finalize_session_with_checkins(
        &self,
        finalization_data: SessionFinalizationData,
    ) -> Result<SessionFinalizationResult>;
}
