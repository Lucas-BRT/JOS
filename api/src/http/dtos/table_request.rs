use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct CreateTableRequestRequest {
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct CreateTableRequestResponse {
    pub id: Uuid,
}
