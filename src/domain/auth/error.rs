#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthDomainError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("insufficient permissions")]
    InsufficientPermissions,
    #[error("invalid token")]
    InvalidToken,
    #[error("token expired")]
    TokenExpired,
    #[error("token not found")]
    TokenNotFound,
}
