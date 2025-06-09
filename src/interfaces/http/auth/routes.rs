use crate::{
    Error, Result,
    error::ValidationError,
    interfaces::http::auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
    state::AppState,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use std::sync::Arc;
use validator::Validate;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> impl IntoResponse {
    if let Err(sanitization_error) = new_user_payload.validate() {
        return Err(Error::Validation(ValidationError::Other(
            sanitization_error,
        )));
    }

    let user = app_state
        .user_service
        .signup(&new_user_payload.into())
        .await;

    match user {
        Ok(user) => {
            let response = UserSignupResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(err) => Err(err),
    }
}

#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login_payload): Json<LoginDto>,
) -> impl IntoResponse {
    if let Err(sanitization_error) = login_payload.validate() {
        return Err(Error::Validation(ValidationError::Other(
            sanitization_error,
        )));
    }

    let jwt_token = app_state
        .user_service
        .login(
            &login_payload.into(),
            &app_state.config.jwt_secret,
            app_state.config.jwt_expiration_duration,
        )
        .await;

    match jwt_token {
        Ok(jwt_token) => Ok((StatusCode::OK, Json(jwt_token)).into_response()),
        Err(err) => Err(err),
    }
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(state.clone())
}
