use crate::{
    Result, interfaces::http::{user::dtos::MeResponse, openapi::{schemas::*, tags::USER_TAG}}, state::AppState, utils::jwt::AuthClaims as Claims,
};
use axum::{
    Json, Router,
    extract::{Multipart, State},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

const MAX_IMAGE_SIZE_BYTES: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_IMAGE_TYPES: [&str; 2] = ["image/jpeg", "image/png"];

pub async fn upload_image(
    State(_app_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    if let Some(field_result) = multipart.next_field().await.transpose() {
        let field = match field_result {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("erro ao ler multipart field: {}", e);
                return Json(json!({
                    "error": "erro ao ler campo do formulário"
                }))
                .into_response();
            }
        };

        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_default();

        if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
            let _ = field.bytes().await;
            tracing::warn!("tipo inválido: {}", content_type);
            return Json(json!({
                "error": "invalid image format",
                "details": "only jpg and png images are allowed"
            }))
            .into_response();
        }

        let timestamp = Utc::now().timestamp();
        let uuid = Uuid::new_v4();
        let final_name = format!("{uuid}_{timestamp}.png");
        let file_path = format!("./public/uploads/{final_name}");

        let mut data = Vec::new();
        let mut stream = field;

        while let Some(chunk_result) = stream.chunk().await.transpose() {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("erro ao ler chunk: {}", e);
                    return Json(json!({
                        "error": "falha ao receber a imagem"
                    }))
                    .into_response();
                }
            };

            data.extend_from_slice(&chunk);
            if data.len() > MAX_IMAGE_SIZE_BYTES {
                tracing::warn!("imagem excedeu limite de tamanho");
                return Json(json!({
                    "error": "image too large",
                    "details": "maximum allowed size is 5MB"
                }))
                .into_response();
            }
        }

        tracing::info!("imagem salva com sucesso: {}", file_path);
        return Json(json!({ "filename": final_name })).into_response();
    }

    Json(json!({
        "error": "no image found"
    }))
    .into_response()
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/users/me",
    tag = USER_TAG,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[axum::debug_handler]
pub async fn me(State(app_state): State<Arc<AppState>>, user: Claims) -> Result<Json<MeResponse>> {
    let user = app_state.user_service.find_by_id(&user.sub).await?;

    Ok(Json(user.into()))
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload-image/{uuid}/image", post(upload_image))
        .route("/me", get(me))
        .with_state(state.clone())
}
