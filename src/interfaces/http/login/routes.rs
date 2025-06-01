use super::dtos::{LoginPayload, LoginResponseDto};
use crate::{
    core::{error::AppError, state::AppState},
    domain::utils::type_wraper::TypeWrapped,
    prelude::AppResult,
    utils::jwt::{Claims, ENCODING_KEY, JWT_ALGORITHM},
};
use axum::{Json, Router, extract::State, routing::post};
use jsonwebtoken::Header;

pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<Json<LoginResponseDto>> {
    let user = app_state
        .user_service
        .find_user_by_username(&payload.username)
        .await?;

    let claims = Claims::new(
        user.id.to_string(),
        user.access_level.to_string(),
        user.email.raw(),
    );

    let token = match jsonwebtoken::encode(&Header::new(JWT_ALGORITHM), &claims, &ENCODING_KEY) {
        Ok(token) => token,
        Err(_) => {
            eprintln!("Erro ao criar token");
            return Err(AppError::InternalServerError);
        }
    };

    Ok(Json(LoginResponseDto::new(token)))
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/", post(login_handler))
        .with_state(state.clone())
}
