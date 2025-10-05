use axum::{
    Json, Router,
    extract::State,
    routing::{delete, put},
};
use std::sync::Arc;
use validator::Validate;

use crate::http::middleware::auth::ClaimsExtractor;
use crate::{http::dtos::*, http::middleware::auth::auth_middleware};
use infrastructure::state::AppState;
use shared::Result;
use shared::error::Error;

#[utoipa::path(
    put,
    path = "/v1/user/profile",
    tag = "user",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UpdateProfileResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 409, description = "Username or email already exists", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn update_profile(
    claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(validation_error.to_string()),
        ));
    }

    // TODO: Implement profile update logic
    // For now, return a placeholder response
    Ok(Json(UpdateProfileResponse {
        id: claims.0.sub,
        username: payload.username.unwrap_or("updated_user".to_string()),
        display_name: payload.display_name.unwrap_or("Updated User".to_string()),
        email: payload.email.unwrap_or("updated@example.com".to_string()),
        joined_at: chrono::Utc::now(),
    }))
}

#[utoipa::path(
    put,
    path = "/v1/user/password",
    tag = "user",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = ChangePasswordResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Current password is incorrect", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn change_password(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(validation_error.to_string()),
        ));
    }

    // Validate password confirmation
    if payload.new_password != payload.confirm_password {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(
                "Password confirmation does not match".to_string(),
            ),
        ));
    }

    // TODO: Implement password change logic
    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/user/account",
    tag = "user",
    request_body = DeleteAccountRequest,
    responses(
        (status = 200, description = "Account deleted successfully", body = DeleteAccountResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Authentication required", body = ErrorResponse),
        (status = 403, description = "Password is incorrect", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn delete_account(
    _claims: ClaimsExtractor,
    State(_app_state): State<Arc<AppState>>,
    Json(payload): Json<DeleteAccountRequest>,
) -> Result<Json<DeleteAccountResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(
            shared::error::ValidationError::ValidationFailed(validation_error.to_string()),
        ));
    }

    // TODO: Implement account deletion logic
    Ok(Json(DeleteAccountResponse {
        message: "Account deleted successfully".to_string(),
    }))
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/profile", put(update_profile))
                .route("/password", put(change_password))
                .route("/account", delete(delete_account))
                .layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .with_state(state)
}
