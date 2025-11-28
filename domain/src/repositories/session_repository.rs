use crate::entities::session_checkin::{SessionFinalizationData, SessionFinalizationResult};
pub use crate::entities::*;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, command: CreateSessionCommand) -> Result<Session>;
    async fn read(&self, command: GetSessionCommand) -> Result<Vec<Session>>;
    async fn update(&self, command: UpdateSessionCommand) -> Result<Session>;
    async fn delete(&self, command: DeleteSessionCommand) -> Result<Session>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>;
    async fn find_by_table_id(&self, table_id: Uuid) -> Result<Vec<Session>>;
    async fn finalize_session_with_checkins(
        &self,
        finalization_data: SessionFinalizationData,
    ) -> Result<SessionFinalizationResult>;
}
