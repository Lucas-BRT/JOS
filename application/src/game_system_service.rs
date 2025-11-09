use domain::entities::*;
use domain::repositories::GameSystemRepository;
use domain::services::IGameSystemService;
use shared::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct GameSystemService<T: GameSystemRepository> {
    game_system_repository: T,
}

impl<T> GameSystemService<T>
where
    T: GameSystemRepository,
{
    pub fn new(game_system_repository: T) -> Self {
        Self {
            game_system_repository,
        }
    }

    pub async fn create<'a>(
        &self,
        command: &CreateGameSystemCommand<'a>,
    ) -> Result<GameSystem, Error> {
        self.game_system_repository.create(command).await
    }

    pub async fn update<'a>(
        &self,
        command: &'a mut UpdateGameSystemCommand<'a>,
    ) -> Result<GameSystem, Error> {
        self.game_system_repository.update(command).await
    }

    pub async fn delete(&self, command: &mut DeleteGameSystemCommand) -> Result<GameSystem, Error> {
        self.game_system_repository.delete(command).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>, Error> {
        self.game_system_repository.find_by_id(id).await
    }
}

#[async_trait::async_trait]
impl<T> IGameSystemService for GameSystemService<T>
where
    T: GameSystemRepository,
{
    async fn create(&self, command: &CreateGameSystemCommand) -> Result<GameSystem, Error> {
        self.game_system_repository.create(command).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<GameSystem>, Error> {
        self.game_system_repository.find_by_id(id).await
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<GameSystem>, Error> {
        self.game_system_repository.find_by_name(name).await
    }

    async fn update(&self, command: &UpdateGameSystemCommand) -> Result<GameSystem, Error> {
        let mut cmd = command.clone();
        self.update(&mut cmd).await
    }

    async fn delete(&self, command: &DeleteGameSystemCommand) -> Result<GameSystem, Error> {
        let mut cmd = command.clone();
        self.delete(&mut cmd).await
    }
}
