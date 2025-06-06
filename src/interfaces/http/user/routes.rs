use super::dtos::{CreateUserDto, CreateUserResponseDto};
use crate::{core::state::AppState, domain::user::entity::User};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};

pub async fn create_user_handler(
    State(app_state): State<AppState>,
    Json(new_user_payload): Json<CreateUserDto>,
) -> Result<Json<CreateUserResponseDto>> {
    let username = app_state
        .user_service
        .create_user(&new_user_payload)
        .await?;

    Ok(Json(CreateUserResponseDto { username }))
}

pub async fn find_user_by_username_handler(
    State(app_state): State<AppState>,
    Path(username_str): Path<String>,
) -> Result<Json<User>> {
    let user = app_state
        .user_service
        .find_user_by_username(&username_str)
        .await?;

    Ok(Json(user))
}

pub async fn get_all_users_handler(State(app_state): State<AppState>) -> Result<Json<Vec<User>>> {
    let users = app_state.user_service.get_all_users().await?;
    Ok(Json(users))
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_all_users_handler))
        .route("/", post(create_user_handler))
        .route("/{username}", get(find_user_by_username_handler))
        .with_state(state.clone())
}
