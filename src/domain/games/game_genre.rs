use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameGenreVo {
    pub id: i32,
    pub name: String,
    pub category: String,
}
