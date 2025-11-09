use crate::http::dtos::*;
use crate::http::error::HttpError;
use crate::http::middleware::auth::auth_middleware;
use axum::routing::*;
use axum::{
    Json, Router,
    extract::{Path, State},
    middleware::from_fn_with_state,
};
use domain::services::IUserService;
use infrastructure::state::AppState;
use shared::Error;
use shared::error::ApplicationError;
use std::sync::Arc;
use uuid::Uuid;

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/:id", get(get_user_by_id))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/v1/user/{id}",
    tag = "users",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "User ID")
    ),
    security(
        ("auth" = [])
    )
)]
pub async fn get_user_by_id(
    State(user_service): State<Arc<dyn IUserService>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>, HttpError> {
    let parsed_user_id = Uuid::parse_str(&user_id).map_err(|_| {
        Error::Application(ApplicationError::InvalidInput {
            message: "Invalid user ID format".to_string(),
        })
    })?;

    let user = user_service.find_by_id(parsed_user_id).await?;

    todo!()
}
