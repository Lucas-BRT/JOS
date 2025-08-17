use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordRequirement {
    pub requirement: String,
    pub expected_value: String,
}

impl PasswordRequirement {
    pub fn new(requirement: String, expected_value: String) -> Self {
        Self {
            requirement,
            expected_value,
        }
    }
}
