use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Informações mínimas sobre um Usuário (para GM).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserMinInfo {
    pub id: Uuid,
    pub display_name: String,
}
