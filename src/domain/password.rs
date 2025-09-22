use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub hash: String,
}

impl Password {
    pub fn new(hash: String) -> Self {
        Self { hash }
    }
}
