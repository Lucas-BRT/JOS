use domain::entities::GetTableCommand;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub struct SearchTablesQuery {
    #[serde(rename = "gmId")]
    pub gm_id: Option<Uuid>,
    #[serde(rename = "gameSystemId")]
    pub game_system_id: Option<Uuid>,
}

impl From<SearchTablesQuery> for GetTableCommand {
    fn from(value: SearchTablesQuery) -> Self {
        Self {
            gm_id: value.gm_id,
            game_system_id: value.game_system_id,
            ..Default::default()
        }
    }
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
