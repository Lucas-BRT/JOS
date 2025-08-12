use crate::{
    Result, interfaces::http::user::dtos::MeResponse, state::AppState, domain::jwt::Claims,
};
use axum::{
    Json, Router,
    extract::State,
    routing::get,
};
use std::sync::Arc;

/// Get current user information
#[utoipa::path(
    get,
    path = "/v1/users/me",
    tag = "users",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User information", body = MeResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value)
    )
)]
#[axum::debug_handler]
pub async fn me(State(app_state): State<Arc<AppState>>, user: Claims) -> Result<Json<MeResponse>> {
    let user = app_state.user_service.find_by_id(&user.sub).await?;

    Ok(Json(user.into()))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/me", get(me))
        .with_state(state.clone())
}
