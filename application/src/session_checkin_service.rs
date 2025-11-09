use domain::entities::*;
use domain::repositories::SessionCheckinRepository;
use shared::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionCheckinService {
    session_checkin_repository: Arc<dyn SessionCheckinRepository>,
}

impl SessionCheckinService {
    pub fn new(session_checkin_repository: Arc<dyn SessionCheckinRepository>) -> Self {
        Self {
            session_checkin_repository,
        }
    }

    pub async fn create<'a>(
        &self,
        command: &'a CreateSessionCheckinCommand<'a>,
    ) -> Result<SessionCheckin, Error> {
        self.session_checkin_repository.create(command).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<SessionCheckin>, Error> {
        self.session_checkin_repository.find_by_id(id).await
    }

    pub async fn find_by_session_intent_id(
        &self,
        session_intent_id: Uuid,
    ) -> Result<Vec<SessionCheckin>, Error> {
        self.session_checkin_repository
            .find_by_session_intent_id(session_intent_id)
            .await
    }

    pub async fn find_by_attendance(&self, attendance: bool) -> Result<Vec<SessionCheckin>, Error> {
        self.session_checkin_repository
            .find_by_attendance(attendance)
            .await
    }

    pub async fn update<'a>(
        &self,
        command: &UpdateSessionCheckinCommand<'a>,
    ) -> Result<SessionCheckin, Error> {
        self.session_checkin_repository.update(command).await
    }

    pub async fn delete(
        &self,
        command: &DeleteSessionCheckinCommand,
    ) -> Result<SessionCheckin, Error> {
        self.session_checkin_repository.delete(command).await
    }
}
