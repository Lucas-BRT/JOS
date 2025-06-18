use crate::interfaces::http::error::ValidationError;
use async_trait::async_trait;
use axum::extract::multipart::Field;

#[async_trait]
pub trait TryFromField: Sized {
    async fn try_from_field(mut field: Field<'_>) -> Result<Self, ValidationError>;
}
