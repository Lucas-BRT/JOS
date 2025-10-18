use crate::http::dtos::{ErrorResponse, UserResponse};
use crate::http::middleware::auth::auth_middleware;
use axum::{
    Json, Router,
    extract::{Path, State},
    middleware::from_fn_with_state,
};
use infrastructure::state::AppState;
use shared::Error as AppError;
use shared::Result;
use shared::error::ApplicationError;
use std::sync::Arc;
use uuid::Uuid;

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/{:id}", axum::routing::get(get_user_by_id))
                .layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/v1/user/{id}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "User ID")
    ),
    security(
        ("token" = [])
    )
)]
pub async fn get_user_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>> {
    let parsed_user_id = Uuid::parse_str(&user_id).map_err(|_| {
        AppError::Application(ApplicationError::InvalidInput {
            message: "Invalid user ID format".to_string(),
        })
    })?;

    let user = app_state.user_service.find_by_id(&parsed_user_id).await?;

    Ok(Json(user.into()))
}
