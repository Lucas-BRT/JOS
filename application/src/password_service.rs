use domain::{repositories::PasswordProvider, services::IPasswordService};
use shared::Error;

#[derive(Clone)]
pub struct PasswordService<T>
where
    T: PasswordProvider,
{
    password_provider: T,
}

impl<T> PasswordService<T>
where
    T: PasswordProvider,
{
    pub fn new(password_provider: T) -> Self {
        Self { password_provider }
    }
}

#[async_trait::async_trait]
impl<T> IPasswordService for PasswordService<T>
where
    T: PasswordProvider,
{
    async fn generate_hash(&self, password: &str) -> Result<String, Error> {
        self.password_provider.generate_hash(password).await
    }

    async fn verify_hash(&self, password: &str, hash: &str) -> Result<bool, Error> {
        self.password_provider.verify_hash(password, hash).await
    }
}
