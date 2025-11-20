use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use crate::http::middleware::auth::auth_middleware;
use axum::middleware::from_fn_with_state;
use axum::{extract::State, http::StatusCode, *};
use domain::auth::Authenticator;
use domain::entities::UpdateUserCommand;
use domain::entities::commands::*;
use infrastructure::state::AppState;
use shared::Error;
use shared::error::ApplicationError;
use shared::*;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use validator::Validate;

#[utoipa::path(post, path = "/login", summary = "User login", tag = "Authentication")]
#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>)> {
    if let Err(validation_error) = login_payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let email = login_payload.email.clone();
    let mut login_command = login_payload.into();

    let user = app_state
        .auth_service
        .user_repository
        .find_by_email(&email)
        .await?
        .ok_or(Error::Application(ApplicationError::InvalidCredentials))?;

    let jwt_token = app_state
        .auth_service
        .authenticate(&mut login_command)
        .await?;

    let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;

    let expires_in = app_state.config.jwt_expiration_duration.num_seconds();

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            user: user.into(),
            token: jwt_token,
            refresh_token,
            expires_in,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/register",
    summary = "User registration",
    tag = "Authentication"
)]
#[axum::debug_handler]
async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err(Error::Validation(validation_error));
    }

    let user = app_state.auth_service.register(&mut payload.into()).await?;
    let jwt_token = app_state
        .auth_service
        .jwt_provider
        .generate_token(&user.id)
        .await?;
    let refresh_token = app_state.auth_service.issue_refresh_token(&user.id).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            user: user.into(),
            token: jwt_token,
            refresh_token,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = "Authentication",
    summary = "User logout",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn logout(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
) -> Result<LogoutResponse> {
    app_state.auth_service.logout(&claims.0.sub).await?;

    Ok(LogoutResponse {
        message: "Logout successful".to_string(),
    })
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "Authentication",
    summary = "Get new JWT token",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn refresh(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<RefreshTokenResponse> {
    let (new_refresh_token, user_id) = app_state
        .auth_service
        .rotate_refresh_token(&payload.refresh_token)
        .await?;

    let new_jwt = app_state
        .auth_service
        .jwt_provider
        .generate_token(&user_id)
        .await?;

    Ok(RefreshTokenResponse {
        token: new_jwt,
        refresh_token: new_refresh_token,
        expires_in: app_state.config.jwt_expiration_duration.num_seconds(),
    })
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "Authentication",
    summary = "Get current user profile",
    security(("auth" = []))
)]
#[axum::debug_handler]
async fn me(
    State(app_state): State<Arc<AppState>>,
    claims: ClaimsExtractor,
) -> Result<UserResponse> {
    let user = app_state
        .auth_service
        .user_repository
        .find_by_id(&claims.0.sub)
        .await?
        .ok_or(Error::Application(ApplicationError::InvalidCredentials))?;

    Ok(user.into())
}

#[utoipa::path(
    put,
    path = "/profile",
    tag = "Authentication",
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

    let mut command = UpdateUserCommand {
        user_id: claims.0.sub,
        username: payload.username.clone().into(),
        email: payload.email.clone().into(),
        ..Default::default()
    };
    let updated_user = app_state.user_service.update(&mut command).await?;

    Ok(Json(UpdateProfileResponse {
        id: claims.0.sub,
        username: updated_user.username,
        email: updated_user.email,
        joined_at: updated_user.created_at,
    }))
}

#[utoipa::path(
    put,
    path = "/password",
    tag = "Authentication",
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

    if payload.new_password != payload.confirm_password {
        let mut errors = validator::ValidationErrors::new();
        let mut error = validator::ValidationError::new("password_mismatch");
        error.message = Some("Password confirmation does not match".into());
        errors.add("confirm_password", error);
        return Err(Error::Validation(errors));
    }

    let mut command = UpdatePasswordCommand {
        user_id: claims.0.sub,
        current_password: payload.current_password,
        new_password: payload.new_password,
    };

    app_state.auth_service.update_password(&mut command).await?;

    Ok(Json(ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
    }))
}

#[utoipa::path(
    delete,
    path = "/account",
    tag = "Authentication",
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

    let mut command = DeleteAccountCommand {
        user_id: claims.0.sub,
        password: payload.password,
    };

    app_state.auth_service.delete_account(&mut command).await?;

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
