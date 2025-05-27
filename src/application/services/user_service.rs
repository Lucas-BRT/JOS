use validator::Validate;

use crate::{
    core::error::AppError,
    domain::user::{dtos::NewUser, entity::User, user_repository::UserRepository},
    interfaces::http::user::dtos::CreateUserDto,
    prelude::AppResult,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, new_user_data: &CreateUserDto) -> AppResult<String> {
        new_user_data
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let new_user = NewUser::try_from(new_user_data).map_err(|e| AppError::Domain(e.into()))?;

        let created_user = self.user_repository.create(&new_user).await?;

        Ok(created_user)
    }

    pub async fn find_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        Ok(self.user_repository.find_by_username(username).await?)
    }

    pub async fn get_all_users(&self) -> AppResult<Vec<User>> {
        Ok(self.user_repository.get_all().await?)
    }
}
