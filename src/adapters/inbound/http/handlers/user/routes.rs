use crate::{
    Result, domain::auth::Claims, interfaces::http::auth::dtos::UserResponse, state::AppState,
    adapters::inbound::http::handlers::user::dtos::{
        MeResponse, UpdateUserDto, ChangePasswordDto, UpdateUserResponse, 
        ChangePasswordResponse, DeleteUserResponse
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, put, delete},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

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

#[utoipa::path(
    put,
    path = "/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn update_user(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<Json<UserResponse>> {
    // Check if user is updating their own profile
    if claims.user_id != user_id {
        return Err(crate::Error::Unauthorized("You can only update your own profile".to_string()));
    }

    if let Err(validation_error) = payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    let user = app_state.user_service.update_user(&user_id, &payload.into()).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    put,
    path = "/v1/users/{id}/password",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    request_body = ChangePasswordDto,
    responses(
        (status = 200, description = "Password changed successfully", body = ChangePasswordResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn change_password(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<ChangePasswordDto>,
) -> Result<Json<ChangePasswordResponse>> {
    // Check if user is changing their own password
    if claims.user_id != user_id {
        return Err(crate::Error::Unauthorized("You can only change your own password".to_string()));
    }

    if let Err(validation_error) = payload.validate() {
        return Err(crate::Error::Validation(validation_error));
    }

    app_state.user_service.change_password(&user_id, &payload.current_password, &payload.new_password).await?;
    
    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = DeleteUserResponse),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 403, description = "Forbidden", body = serde_json::Value)
    )
)]
pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<Json<DeleteUserResponse>> {
    // Check if user is deleting their own account
    if claims.user_id != user_id {
        return Err(crate::Error::Unauthorized("You can only delete your own account".to_string()));
    }

    app_state.user_service.delete_user(&user_id).await?;
    
    Ok(Json(DeleteUserResponse {
        message: "User deleted successfully".to_string(),
    }))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}/password", put(change_password))
        .route("/{id}", delete(delete_user))
        .with_state(state.clone())
}
