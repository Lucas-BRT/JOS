use domain::entities::*;
use domain::repositories::UserRepository;
use domain::services::user_service::IUserService;
use shared::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> UserService<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl<T> IUserService for UserService<T>
where
    T: UserRepository,
{
    async fn create(&self, command: &CreateUserCommand) -> Result<User, Error> {
        self.user_repository.create(command).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        self.user_repository.find_by_id(id).await
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        todo!()
    }

    async fn update(&self, command: &UpdateUserCommand) -> Result<User, Error> {
        self.user_repository.update(command).await
    }

    async fn delete(&self, command: &mut DeleteUserCommand) -> Result<User, Error> {
        self.user_repository.delete(command).await
    }
}
