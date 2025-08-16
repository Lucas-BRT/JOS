use crate::{
    Result,
    domain::user::{
        UpdateUserCommand,
        commands::{CreateUserCommand, LoginUserCommand},
        entity::User,
    },
};

#[async_trait::async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, payload: &LoginUserCommand) -> Result<String>;
    async fn register(&self, payload: &mut CreateUserCommand) -> Result<User>;
    async fn update_password(&self, payload: &mut UpdateUserCommand) -> Result<()>;
}
