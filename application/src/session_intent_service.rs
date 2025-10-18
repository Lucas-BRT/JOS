use domain::entities::*;
use domain::repositories::SessionIntentRepository;
use shared::Result;
use shared::error::DomainError;
use shared::error::Error;
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

    pub async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent> {
        self.session_intent_repository.create(command).await
    }

    pub async fn get(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>> {
        self.session_intent_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<SessionIntent> {
        let command = GetSessionIntentCommand {
            id: Some(*id),
            ..Default::default()
        };
        let session_intents = self.session_intent_repository.read(command).await?;

        session_intents
            .into_iter()
            .next()
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "SessionIntent",
                entity_id: id.to_string(),
            }))
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<SessionIntent>> {
        let command = GetSessionIntentCommand {
            user_id: Some(*user_id),
            ..Default::default()
        };
        self.session_intent_repository.read(command).await
    }

    pub async fn find_by_session_id(&self, session_id: &Uuid) -> Result<Vec<SessionIntent>> {
        let command = GetSessionIntentCommand {
            session_id: Some(*session_id),
            ..Default::default()
        };
        self.session_intent_repository.read(command).await
    }

    pub async fn update(&self, command: UpdateSessionIntentCommand) -> Result<SessionIntent> {
        self.session_intent_repository.update(command).await
    }

    pub async fn delete(&self, command: DeleteSessionIntentCommand) -> Result<SessionIntent> {
        self.session_intent_repository.delete(command).await
    }
}
