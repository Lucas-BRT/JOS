use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Username(String);
