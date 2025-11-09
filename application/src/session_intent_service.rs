use domain::entities::*;
use domain::repositories::SessionIntentRepository;
use shared::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionIntentService {
    session_intent_repository: Arc<dyn SessionIntentRepository>,
}

impl SessionIntentService {
    pub fn new(session_intent_repository: Arc<dyn SessionIntentRepository>) -> Self {
        Self {
            session_intent_repository,
        }
    }

    pub async fn create(
        &self,
        command: CreateSessionIntentCommand,
    ) -> Result<SessionIntent, Error> {
        self.session_intent_repository.create(&command).await
    }

    pub async fn get(
        &self,
        command: &GetSessionIntentCommand,
    ) -> Result<Vec<SessionIntent>, Error> {
        self.session_intent_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>, Error> {
        self.session_intent_repository.find_by_id(id).await
    }

    pub async fn find_by_user_id(&self, id: Uuid) -> Result<Vec<SessionIntent>, Error> {
        self.session_intent_repository.find_by_user_id(id).await
    }

    pub async fn find_by_session_id(&self, id: Uuid) -> Result<Vec<SessionIntent>, Error> {
        self.session_intent_repository.find_by_session_id(id).await
    }

    pub async fn update(
        &self,
        command: &UpdateSessionIntentCommand,
    ) -> Result<SessionIntent, Error> {
        self.session_intent_repository.update(command).await
    }

    pub async fn delete(
        &self,
        command: &DeleteSessionIntentCommand,
    ) -> Result<SessionIntent, Error> {
        self.session_intent_repository.delete(command).await
    }
}
