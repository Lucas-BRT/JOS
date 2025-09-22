use crate::infrastructure::error::SetupError;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Domain error: {0}")]
    Domain(crate::domain::error::DomainError),
    #[error("Application error: {0}")]
    Application(crate::application::error::ApplicationError),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Setup error: {0}")]
    Setup(SetupError),
}

impl From<crate::domain::error::DomainError> for Error {
    fn from(err: crate::domain::error::DomainError) -> Self {
        Error::Domain(err)
    }
}
