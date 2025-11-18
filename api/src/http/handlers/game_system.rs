use crate::http::{
    dtos::{
        CreateGameSystemRequest, CreateGameSystemRespose, CreateTableResponse, GameSystemResponse,
    },
    middleware::auth::auth_middleware,
};
use axum::{Json, extract::State, middleware::from_fn_with_state};
use domain::entities::GetGameSystemCommand;
use infrastructure::state::AppState;
use shared::{Error, Result};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/",
    tag = "game_systems",
    request_body = CreateGameSystemRequest,
    security(("auth" = [])),
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

#[utoipa::path(
    get,
    path = "/",
    tag = "game_systems",
    security(("auth" = [])),
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
