use base64::Engine;
use chrono::Utc;
use domain::auth::*;
use domain::entities::*;
use domain::repositories::*;
use domain::services::IAuthService;
use rand::RngCore;
use shared::Error;
use shared::error::ApplicationError;
use shared::error::DomainError;
use uuid::{NoContext, Uuid};

#[derive(Clone)]
pub struct AuthService<T, U, V, W>
where
    T: UserRepository,
    U: PasswordProvider,
    V: TokenProvider,
    W: RefreshTokenRepository,
{
    pub user_repository: T,
    pub password_provider: U,
    pub jwt_provider: V,
    pub refresh_token_repository: W,
}

impl<T, U, V, W> AuthService<T, U, V, W>
where
    T: UserRepository,
    U: PasswordProvider,
    V: TokenProvider,
    W: RefreshTokenRepository,
{
    pub fn new(
        user_repository: T,
        password_provider: U,
        jwt_provider: V,
        refresh_token_repository: W,
    ) -> Self {
        Self {
            user_repository,
            password_provider,
            jwt_provider,
            refresh_token_repository,
        }
    }
}

#[async_trait::async_trait]
impl<T, U, V, W> IAuthService for AuthService<T, U, V, W>
where
    T: UserRepository,
    U: PasswordProvider,
    V: TokenProvider,
    W: RefreshTokenRepository,
{
    async fn decode_access_token(&self, token: &str) -> Result<Claims, Error> {
        self.jwt_provider.decode_token(token).await
    }

    async fn issue_refresh_token(&self, user_id: Uuid) -> Result<String, Error> {
        self.refresh_token_repository
            .delete_by_user(user_id)
            .await?;

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token_str = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        let rt = RefreshToken {
            id: Uuid::new_v7(uuid::Timestamp::now(NoContext)),
            user_id,
            token: token_str.clone(),
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
        };

        self.refresh_token_repository.create(&rt).await?;
        Ok(token_str)
    }

    async fn rotate_refresh_token(&self, old_token: &str) -> Result<(String, Uuid), Error> {
        let existing = self
            .refresh_token_repository
            .find_by_token(old_token)
            .await?;

        let record = match existing {
            Some(r) => r,
            None => {
                return Err(Error::Application(ApplicationError::InvalidCredentials));
            }
        };

        if record.expires_at < Utc::now() {
            self.refresh_token_repository
                .delete_by_token(old_token)
                .await?;

            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        self.refresh_token_repository
            .delete_by_token(old_token)
            .await?;

        let new_token = self.issue_refresh_token(record.user_id).await?;
        Ok((new_token, record.user_id))
    }

    async fn authenticate(&self, payload: &LoginUserCommand) -> Result<String, Error> {
        let (id, password) = match self.user_repository.find_by_email(payload.email).await? {
            Some(user) => (user.id, user.password),
            None => {
                return Err(Error::Application(ApplicationError::InvalidCredentials));
            }
        };

        if !self
            .password_provider
            .verify_hash(payload.password, &password)
            .await?
        {
            return Err(Error::Application(ApplicationError::InvalidCredentials));
        }

        let jwt_token = self.jwt_provider.generate_token(id).await?;

        Ok(jwt_token)
    }

    async fn register(&self, payload: &CreateUserCommand) -> Result<User, Error> {
        let payload = &mut payload.clone();
        payload.password = self
            .password_provider
            .generate_hash(&payload.password)
            .await?
            .to_string();

        let created_user = self.user_repository.create(payload).await?;

        Ok(created_user)
    }

    async fn update_password(&self, payload: &UpdatePasswordCommand) -> Result<(), Error> {
        let user = self
            .user_repository
            .find_by_id(payload.user_id)
            .await?
            .ok_or(Error::Domain(DomainError::UserNotFound))?;

        if !self
            .password_provider
            .verify_hash(payload.current_password, &user.password)
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        let new_password_hash = self
            .password_provider
            .generate_hash(payload.new_password)
            .await?;

        let command = UpdateUserCommand {
            user_id: payload.user_id,
            password: Some(&new_password_hash),
            ..Default::default()
        };

        self.user_repository.update(&command).await?;

        Ok(())
    }

    async fn logout(&self, user_id: Uuid) -> Result<(), Error> {
        Ok(self
            .refresh_token_repository
            .delete_by_user(user_id)
            .await?)
    }

    async fn delete_account(&self, command: &DeleteAccountCommand) -> Result<(), Error> {
        let user = self
            .user_repository
            .find_by_id(command.user_id)
            .await?
            .ok_or(Error::Domain(DomainError::UserNotFound))?;

        if !self
            .password_provider
            .verify_hash(command.password, &user.password)
            .await?
        {
            return Err(Error::Application(ApplicationError::IncorrectPassword));
        }

        self.user_repository.delete_by_id(command.user_id).await?;

        Ok(())
    }
}
