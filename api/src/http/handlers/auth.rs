use crate::http::dtos::LoginResponse;
use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use crate::http::middleware::auth::auth_middleware;
use application::user_service::UpdateProfileCommand;
use axum::middleware::from_fn_with_state;
use axum::{extract::State, http::StatusCode, *};
use domain::auth::*;
use domain::entities::commands::DeleteAccountCommand;
use infrastructure::state::AppState;
use shared::Error;
use shared::*;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use validator::Validate;

#[utoipa::path(post, path = "/login", summary = "User login", tag = "auth")]
#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>)> {
    if let Err(validation_error) = login_payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let login_command = LoginCommand {
        email: login_payload.email,
        password: login_payload.password,
    };

    let auth_response = app_state.auth_service.login(login_command).await?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            user: auth_response.user.into(),
            token: auth_response.access_token,
            refresh_token: auth_response.refresh_token,
            expires_in: auth_response.expires_in,
        }),
    ))
}

#[utoipa::path(post, path = "/register", summary = "User registration", tag = "auth")]
#[axum::debug_handler]
async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = RegisterCommand {
        username: payload.username,
        email: payload.email,
        password: payload.password,
    };

    let auth_response = app_state.auth_service.register(command).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            user: auth_response.user.into(),
            token: auth_response.access_token,
            refresh_token: auth_response.refresh_token,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = "auth",
    summary = "User logout",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn logout(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
) -> Result<LogoutResponse> {
    let command = LogoutCommand {
        user_id: claims.get_user_id(),
    };

    app_state.auth_service.logout(command).await?;

    Ok(LogoutResponse {
        message: "Logout successful".to_string(),
    })
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "auth",
    summary = "Get new JWT token",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn refresh(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<RefreshTokenResponse> {
    let command = RefreshTokenCommand {
        token: payload.refresh_token,
    };

    let refresh_response = app_state.auth_service.refresh_token(command).await?;

    Ok(RefreshTokenResponse {
        token: refresh_response.access_token,
        refresh_token: refresh_response.refresh_token,
        expires_in: refresh_response.expires_in,
    })
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "auth",
    summary = "Get current user profile",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn me(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
) -> Result<UserResponse> {
    let profile = app_state
        .user_service
        .get_user_profile(claims.0.sub)
        .await?;

    Ok(UserResponse {
        id: profile.id,
        username: profile.username,
        email: profile.email,
        joined_at: profile.joined_at,
    })
}

#[utoipa::path(
    put,
    path = "/profile",
    tag = "auth",
    summary = "Update user profile",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn update_profile(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = UpdateProfileCommand {
        username: payload.username,
        email: payload.email,
    };

    let profile = app_state
        .user_service
        .update_profile(claims.0.sub, command)
        .await?;

    Ok(Json(UpdateProfileResponse {
        id: profile.id,
        username: profile.username,
        email: profile.email,
        joined_at: profile.joined_at,
    }))
}

#[utoipa::path(
    put,
    path = "/password",
    tag = "auth",
    summary = "Change user password",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn change_password(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = ChangePasswordCommand {
        current_password: payload.current_password,
        new_password: payload.new_password,
        confirm_password: payload.confirm_password,
    };

    app_state
        .auth_service
        .change_password(claims.get_user_id(), command)
        .await?;

    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/account",
    tag = "auth",
    summary = "Delete user account",
    security(("auth" = []))
)]
#[axum::debug_handler]
pub async fn delete_account(
    claims: ClaimsExtractor,
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<DeleteAccountRequest>,
) -> Result<Json<DeleteAccountResponse>> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let command = DeleteAccountCommand {
        user_id: claims.0.sub,
        password: payload.password,
    };

    app_state.user_service.delete_account(command).await?;

    Ok(Json(DeleteAccountResponse {
        message: "Account deleted successfully".to_string(),
    }))
}

pub fn auth_routes(state: Arc<AppState>) -> OpenApiRouter {
    let public = OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login));

    let protected = OpenApiRouter::new()
        .routes(routes!(logout))
        .routes(routes!(refresh))
        .routes(routes!(me))
        .routes(routes!(update_profile))
        .routes(routes!(change_password))
        .routes(routes!(delete_account))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    OpenApiRouter::new()
        .nest("/auth", OpenApiRouter::new().merge(public).merge(protected))
        .with_state(state)
}
