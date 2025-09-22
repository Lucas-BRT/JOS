use crate::Result;
use crate::domain::entities::{
    CreateSessionCheckinCommand, DeleteSessionCheckinCommand, UpdateSessionCheckinCommand,
};
use crate::domain::entities::{GetSessionCheckinCommand, SessionCheckin};

#[async_trait::async_trait]
pub trait SessionCheckinRepository: Send + Sync {
    async fn create(&self, command: CreateSessionCheckinCommand) -> Result<SessionCheckin>;
    async fn read(&self, command: GetSessionCheckinCommand) -> Result<Vec<SessionCheckin>>;
    async fn update(&self, command: UpdateSessionCheckinCommand) -> Result<SessionCheckin>;
    async fn delete(&self, command: DeleteSessionCheckinCommand) -> Result<SessionCheckin>;
}
