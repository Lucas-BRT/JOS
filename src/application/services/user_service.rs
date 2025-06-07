use crate::{
    Result,
    domain::user::{dtos::CreateUserCommand, entity::User, user_repository::UserRepository},
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

    pub async fn create(&self, _new_user_data: &CreateUserCommand) -> Result<String> {
        todo!()
    }

    pub async fn find_by_username(&self, _username: &str) -> Result<User> {
        todo!()
    }

    pub async fn get(&self) -> Result<Vec<User>> {
        Ok(self.user_repository.get_all().await?)
    }
}
