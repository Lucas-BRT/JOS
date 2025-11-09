use crate::http::{dtos::*, error::HttpError, middleware::auth::auth_middleware};
use axum::{Json, Router, extract::State, middleware::from_fn_with_state, routing::*};
use domain::services::IGameSystemService;
use infrastructure::state::AppState;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/v1/game_systems",
    tag = "game_systems",
    request_body = CreateGameSystemRequest,
    responses(
        (status = 200, description = "", body = CreateGameSystemRespose),
    )
)]
#[axum::debug_handler]
async fn create_game_system(
    State(app_app): State<Arc<dyn IGameSystemService>>,
    Json(payload): Json<CreateGameSystemRequest>,
) -> Result<Json<CreateTableResponse>, HttpError> {
    /*
    let command = CreateGameSystemCommand {
        id: Uuid::new_v4(),
        name: payload.name,
    };

    let created = app_app.game_system_service.create(&command).await?;

    Ok(Json(CreateGameSystemRespose { id: created.id }))
    */
    todo!()
}

#[utoipa::path(
    get,
    path = "/v1/game_systems",
    tag = "game_systems",
    request_body = CreateGameSystemRequest,
    responses(
    )
)]
#[axum::debug_handler]
async fn get_game_systems(
    State(app_app): State<Arc<dyn IGameSystemService>>,
) -> Result<Json<Vec<GameSystemResponse>>, HttpError> {
    /*
    let systems = app_app
        .game_system_service
        .get(&mut GetGameSystemCommand::default())
        .await?
        .iter()
        .map(GameSystemResponse::from)
        .collect();

    Ok(Json(systems))
    */
    todo!()
}

pub fn game_system_routes(state: AppState) -> Router {
    let protected = Router::new()
        .route("/", post(create_game_system))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    let public = Router::new().route("/", get(get_game_systems));

    Router::new()
        .nest(
            "/game_systems",
            Router::new().merge(protected).merge(public),
        )
        .with_state(state)
}
