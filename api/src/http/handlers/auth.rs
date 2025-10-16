use crate::http::dtos::*;
use crate::http::middleware::auth::ClaimsExtractor;
use crate::http::middleware::auth::auth_middleware;
use axum::middleware::from_fn_with_state;
use axum::{extract::State, http::StatusCode, routing::*, *};
use domain::auth::Authenticator;
use domain::entities::UpdateUserCommand;
use domain::entities::commands::*;
use infrastructure::state::AppState;
use shared::Error;
use shared::error::ApplicationError;
use shared::*;
use std::sync::Arc;
use validator::Validate;

// Conversion implementations
impl From<LoginRequest> for LoginUserCommand {
    fn from(req: LoginRequest) -> Self {
        LoginUserCommand {
            email: req.email,
            password: req.password,
        }
    }
}

impl From<RegisterRequest> for CreateUserCommand {
    fn from(req: RegisterRequest) -> Self {
        CreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = serde_json::Value),
        (status = 400, description = "Missing required fields", body = serde_json::Value)
    )
)]
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
    path = "/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created successfully", body = RegisterResponse),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 409, description = "Email or username already exists", body = serde_json::Value)
    )
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
    path = "/v1/auth/logout",
    tag = "auth",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
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
    path = "/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token", body = serde_json::Value)
    )
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
    path = "/v1/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User data retrieved", body = UserResponse),
        (status = 401, description = "Invalid token", body = serde_json::Value)
    )
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
    path = "/v1/auth/profile",
    tag = "auth",
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
        password: payload.password.clone().into(),
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
    path = "/v1/auth/password",
    tag = "auth",
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
    path = "/v1/auth/account",
    tag = "auth",
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
        return Err(Error::Validation(validation_error));
    }

    // TODO: Implement account deletion logic
    Ok(Json(DeleteAccountResponse {
        message: "Account deleted successfully".to_string(),
    }))
}

pub fn auth_routes(state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh));

    let protected = Router::new()
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/profile", put(update_profile))
        .route("/password", put(change_password))
        .route("/account", delete(delete_account))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .nest("/auth", Router::new().merge(public).merge(protected))
        .with_state(state)
}
