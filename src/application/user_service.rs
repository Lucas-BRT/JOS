use crate::Result;
use crate::domain::user::dtos::MeResponse;
use crate::domain::user::entity::User;
use crate::domain::user::search_commands::UserFilters;
use crate::domain::user::user_repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn find_users(&self, filters: &UserFilters) -> Result<Vec<User>> {
        self.user_repository.get_all(filters).await
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<User> {
        self.user_repository.find_by_id(user_id).await
    }

    pub async fn get_self_user_info(&self, user_id: &Uuid) -> Result<MeResponse> {
        let user = self.user_repository.find_by_id(user_id).await?;
        Ok(user.into())
    }
}
