use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DisplayName(String);
