use crate::{
    Error, Result,
    domain::user::{dtos::CreateUserCommand, entity::User, user_repository::UserRepository},
    error::ValidationError,
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

    pub async fn signup(&self, new_user_data: &CreateUserCommand) -> Result<User> {
        if new_user_data.password != new_user_data.confirm_password {
            return Err(Error::Validation(ValidationError::PasswordMismatch));
        }

        let user = self.user_repository.create(new_user_data).await?;
        Ok(user)
    }

    pub async fn find_by_username(&self, _username: &str) -> Result<User> {
        todo!()
    }

    pub async fn get(&self) -> Result<Vec<User>> {
        Ok(self.user_repository.get_all().await?)
    }
}
