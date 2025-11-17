pub mod postgres;
pub use postgres::*;
use sqlx::PgPool;

pub type Db = PgPool;
