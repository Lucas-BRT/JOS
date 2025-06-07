use crate::Error;
use axum::response::IntoResponse;
use sqlx::PgPool;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;
pub type Db = Arc<PgPool>;
