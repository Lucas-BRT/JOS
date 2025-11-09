use axum::{Json, response::IntoResponse};
use domain::entities::GameSystem;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::http::dtos::CreateTableResponse;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateGameSystemRequest {
    #[validate(length(max = 80))]
    name: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateGameSystemRespose {
    id: Uuid,
}

impl IntoResponse for CreateTableResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self.id).into_response()
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct GameSystemResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<&GameSystem> for GameSystemResponse {
    fn from(value: &GameSystem) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
        }
    }
}
