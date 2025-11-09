use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ISessionCheckinService: Send + Sync {
    async fn create(&self, command: &CreateSessionCheckinCommand) -> Result<SessionCheckin, Error>;
    async fn get(&self, command: &GetSessionCheckinCommand) -> Result<Vec<SessionCheckin>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<SessionCheckin, Error>;
    async fn find_by_session_intent_id(
        &self,
        session_intent_id: Uuid,
    ) -> Result<Vec<SessionCheckin>, Error>;
    async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>, Error>;
    async fn update(&self, command: &UpdateSessionCheckinCommand) -> Result<SessionCheckin, Error>;
    async fn delete(&self, command: &DeleteSessionCheckinCommand) -> Result<SessionCheckin, Error>;
}
