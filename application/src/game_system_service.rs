use domain::entities::*;
use domain::repositories::GameSystemRepository;
use shared::Result;
use shared::error::{DomainError, Error};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GameSystemService {
    game_system_repository: Arc<dyn GameSystemRepository>,
}

impl GameSystemService {
    pub fn new(game_system_repository: Arc<dyn GameSystemRepository>) -> Self {
        Self {
            game_system_repository,
        }
    }

    pub async fn create(&self, command: &mut CreateGameSystemCommand) -> Result<GameSystem> {
        self.game_system_repository.create(command).await
    }

    pub async fn get(&self, command: &mut GetGameSystemCommand) -> Result<Vec<GameSystem>> {
        self.game_system_repository.read(command).await
    }

    pub async fn update(&self, command: &mut UpdateGameSystemCommand) -> Result<GameSystem> {
        self.game_system_repository.update(command).await
    }

    pub async fn delete(&self, command: &mut DeleteGameSystemCommand) -> Result<GameSystem> {
        self.game_system_repository.delete(command).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<GameSystem> {
        self.game_system_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| {
                Error::Domain(DomainError::EntityNotFound {
                    entity_type: "GameSystem",
                    entity_id: id.to_string(),
                })
            })
    }
}
