pub mod postgres;

use sqlx::PgPool;

pub type Db = PgPool;
