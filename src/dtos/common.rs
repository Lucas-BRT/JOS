use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SearchTablesQuery {
    pub search: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    pub errors: Option<Vec<ValidationError>>,
    pub code: Option<String>,
}
