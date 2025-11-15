use crate::http::{dtos::CreateTableResponse, middleware::auth::auth_middleware};
use axum::{Json, extract::State, middleware::from_fn_with_state, response::IntoResponse};
use domain::entities::{CreateGameSystemCommand, GameSystem, GetGameSystemCommand};
use infrastructure::state::AppState;
use serde::*;
use shared::{Error, Result};
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
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

impl IntoResponse for CreateTableResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self.id).into_response()
    }
}

impl From<CreateGameSystemRequest> for CreateGameSystemCommand {
    fn from(value: CreateGameSystemRequest) -> Self {
        Self { name: value.name }
    }
}

#[utoipa::path(
    post,
    path = "/",
    tag = "game_systems",
    request_body = CreateGameSystemRequest,
    responses(
        (status = 200, description = "", body = CreateGameSystemRespose),
    )
)]
#[axum::debug_handler]
async fn create_game_system(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateGameSystemRequest>,
) -> Result<CreateTableResponse> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let id = app_state
        .game_system_service
        .create(&mut payload.into())
        .await?
        .id;

    Ok(CreateTableResponse { id })
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

#[utoipa::path(
    get,
    path = "/",
    tag = "game_systems",
    responses(
        (status = 200, description = "", body = Vec<GameSystemResponse>),
    )
)]
#[axum::debug_handler]
async fn get_game_systems(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<GameSystemResponse>>> {
    let systems = app_state
        .game_system_service
        .get(&mut GetGameSystemCommand::default())
        .await?
        .iter()
        .map(GameSystemResponse::from)
        .collect();

    Ok(Json(systems))
}

pub fn game_system_routes(state: Arc<AppState>) -> OpenApiRouter {
    let protected = OpenApiRouter::new()
        .routes(routes!(create_game_system))
        .routes(routes!(get_game_systems))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    OpenApiRouter::new()
        .nest("/game_systems", OpenApiRouter::new().merge(protected))
        .with_state(state)
}
