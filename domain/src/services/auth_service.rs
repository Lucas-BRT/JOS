use crate::auth::Claims;
use crate::entities::*;
use shared::Error;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait IAuthService: Send + Sync {
    async fn decode_access_token(&self, token: &str) -> Result<Claims, Error>;
    async fn issue_refresh_token(&self, user_id: Uuid) -> Result<String, Error>;
    async fn rotate_refresh_token(&self, old_token: &str) -> Result<(String, Uuid), Error>;
    async fn authenticate(&self, command: &LoginUserCommand) -> Result<String, Error>;
    async fn register(&self, command: &CreateUserCommand) -> Result<User, Error>;
    async fn update_password(&self, command: &UpdatePasswordCommand) -> Result<(), Error>;
    async fn logout(&self, user_id: Uuid) -> Result<(), Error>;
    async fn delete_account(&self, command: &DeleteAccountCommand) -> Result<(), Error>;
}
