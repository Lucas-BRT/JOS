use crate::http::dtos::UserResponse;
use crate::http::middleware::auth::auth_middleware;
use axum::{
    Json,
    extract::{Path, State},
    middleware::from_fn_with_state,
};
use infrastructure::state::AppState;
use shared::Error;
use shared::Result;
use shared::error::ApplicationError;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/{user_id}",
    security(("auth" = [])),
    summary = "Get details about a existing user",
    tag = "user",
)]
pub async fn get_user_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = app_state
        .user_service
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| Error::Application(ApplicationError::InvalidCredentials))?;

    Ok(Json(user.into()))
}

pub fn user_routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .nest(
            "/user",
            OpenApiRouter::new()
                .routes(routes!(get_user_by_id))
                .layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .with_state(state)
}
