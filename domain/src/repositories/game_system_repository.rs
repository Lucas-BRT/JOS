use crate::{entities::*, repositories::Repository};
use shared::Result;

#[async_trait::async_trait]
pub trait GameSystemRepository:
    Repository<
        GameSystem,
        CreateGameSystemCommand,
        UpdateGameSystemCommand,
        GetGameSystemCommand,
        DeleteGameSystemCommand,
    > + Send
    + Sync
{
    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>>;
}
