use crate::http::dtos::CreateTableResponse;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use domain::entities::{CreateGameSystemCommand, GameSystem};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateGameSystemRequest {
    #[validate(length(max = 80))]
    name: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateGameSystemRespose {
    id: Uuid,
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

impl IntoResponse for CreateTableResponse {
    fn into_response(self) -> Response {
        Json(self.id).into_response()
    }
}

impl From<CreateGameSystemRequest> for CreateGameSystemCommand {
    fn from(value: CreateGameSystemRequest) -> Self {
        Self { name: value.name }
    }
}
