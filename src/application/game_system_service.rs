use crate::Result;
use crate::domain::entities::*;
use crate::domain::repositories::GameSystemRepository;
use std::sync::Arc;

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
}
