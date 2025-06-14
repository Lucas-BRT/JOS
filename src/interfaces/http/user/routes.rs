use crate::{
    Result, interfaces::http::user::dtos::MeResponse, state::AppState, utils::jwt::Claims,
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

const MAX_IMAGE_SIZE: usize = 1024 * 1024 * 5; // 5MB
const ALLOWED_IMAGE_TYPES: [&str; 2] = ["image/jpeg", "image/png"];

pub async fn upload_image(
    State(app_state): State<Arc<AppState>>,
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

        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or("image".to_string());
        let timestamp = Utc::now().timestamp();
        let uuid = Uuid::new_v4();
        let final_name = format!("{}_{}.png", uuid, timestamp);
        let file_path = format!("./public/uploads/{}", final_name);

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
            if data.len() > MAX_IMAGE_SIZE {
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
