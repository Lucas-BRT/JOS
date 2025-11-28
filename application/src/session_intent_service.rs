use domain::entities::*;
use domain::repositories::{
    SessionIntentRepository, TableMemberRepository, TableRepository, UserRepository,
};
use shared::Result;
use shared::error::DomainError;
use shared::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionIntentService {
    session_intent_repository: Arc<dyn SessionIntentRepository>,
    user_repository: Arc<dyn UserRepository>,
    table_repository: Arc<dyn TableRepository>,
    table_member_repository: Arc<dyn TableMemberRepository>,
}

impl SessionIntentService {
    pub fn new(
        session_intent_repository: Arc<dyn SessionIntentRepository>,
        user_repository: Arc<dyn UserRepository>,
        table_repository: Arc<dyn TableRepository>,
        table_member_repository: Arc<dyn TableMemberRepository>,
    ) -> Self {
        Self {
            session_intent_repository,
            user_repository,
            table_repository,
            table_member_repository,
        }
    }

    pub async fn create_with_validation(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        status: IntentStatus,
    ) -> Result<()> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: user_id.to_string(),
            }))?;

        let table = self
            .table_repository
            .find_by_session_id(&session_id)
            .await?
            .ok_or(Error::Domain(DomainError::BusinessRuleViolation {
                message: "Can only create session intent in a table that already exists"
                    .to_string(),
            }))?;

        let table_members = self
            .table_member_repository
            .find_by_table_id(table.id)
            .await?;

        if !table_members.iter().any(|member| member.user_id == user.id) {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "User must be a member of the table to create a session intent"
                    .to_string(),
            }));
        }

        let command = CreateSessionIntentCommand {
            player_id: user.id,
            session_id,
            status,
        };

        self.create(command).await?;
        Ok(())
    }

    pub async fn get_for_session_with_validation(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<Vec<SessionIntent>> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: user_id.to_string(),
            }))?;

        let table = self
            .table_repository
            .find_by_session_id(&session_id)
            .await?
            .ok_or(Error::Domain(DomainError::BusinessRuleViolation {
                message: "Can only access session intents in a table that exists".to_string(),
            }))?;

        let table_members = self
            .table_member_repository
            .find_by_table_id(table.id)
            .await?;

        if !table_members.iter().any(|member| member.user_id == user.id) {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "User must be a member of the table to access session intents".to_string(),
            }));
        }

        self.find_by_session_id(&session_id).await
    }

    pub async fn update_with_validation(
        &self,
        user_id: Uuid,
        intent_id: Uuid,
        status: Option<IntentStatus>,
    ) -> Result<()> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(Error::Domain(DomainError::EntityNotFound {
                entity_type: "User",
                entity_id: user_id.to_string(),
            }))?;

        let intent = self
            .find_intent_by_id(intent_id)
            .await?
            .ok_or(Error::Domain(DomainError::BusinessRuleViolation {
                message: "session intent not found".into(),
            }))?;

        if intent.user_id != user.id {
            return Err(Error::Domain(DomainError::BusinessRuleViolation {
                message: "can only update own session intents".into(),
            }));
        }

        let command = UpdateSessionIntentCommand {
            id: intent_id,
            status,
        };

        self.update(command).await?;
        Ok(())
    }

    pub async fn create(&self, command: CreateSessionIntentCommand) -> Result<SessionIntent> {
        self.session_intent_repository.create(command).await
    }

    pub async fn get(&self, command: GetSessionIntentCommand) -> Result<Vec<SessionIntent>> {
        self.session_intent_repository.read(command).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<SessionIntent>> {
        let command = GetSessionIntentCommand {
            id: Some(*id),
            ..Default::default()
        };
        let session_intents = self.session_intent_repository.read(command).await?;
        Ok(session_intents.into_iter().next())
    }

    pub async fn find_intent_by_id(&self, id: Uuid) -> Result<Option<SessionIntent>> {
        self.session_intent_repository.find_by_id(id).await
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
