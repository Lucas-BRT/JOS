use crate::{
    Result,
    domain::{auth::Claims, user::dtos::MeResponse},
    interfaces::http::auth::dtos::UserResponse,
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/users/me",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User information", body = MeResponse),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
#[axum::debug_handler]
pub async fn me(State(app_state): State<Arc<AppState>>, user: Claims) -> Result<Json<MeResponse>> {
    let user = app_state.user_service.get_self_user_info(&user.sub).await?;

    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User information", body = UserResponse),
        (status = 401, description = "Unauthorized", body = Value)
    )
)]
pub async fn get_user_by_id(
    State(app_state): State<Arc<AppState>>,
    _: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = app_state.user_service.get_user(&user_id).await?;

    Ok(Json(user.into()))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/{id}", get(get_user_by_id))
        .with_state(state.clone())
}
