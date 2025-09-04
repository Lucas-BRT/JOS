use crate::shared::Error;
use sqlx::PgPool;

pub type Result<T> = std::result::Result<T, Error>;
pub type Db = PgPool;
