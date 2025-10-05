use crate::entities::*;
use shared::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionCheckinService: Send + Sync {
    async fn create(&self, command: &CreateSessionCheckinCommand) -> Result<SessionCheckin>;
    async fn get(&self, command: &GetSessionCheckinCommand) -> Result<Vec<SessionCheckin>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<SessionCheckin>;
    async fn find_by_session_intent_id(
        &self,
        session_intent_id: &Uuid,
    ) -> Result<Vec<SessionCheckin>>;
    async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>>;
    async fn update(&self, command: &UpdateSessionCheckinCommand) -> Result<SessionCheckin>;
    async fn delete(&self, command: &DeleteSessionCheckinCommand) -> Result<SessionCheckin>;
}
