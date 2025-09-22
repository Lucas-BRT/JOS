use crate::shared::Error;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub type Result<T> = std::result::Result<T, Error>;
pub type Db = PgPool;
pub type Date = DateTime<Utc>;
