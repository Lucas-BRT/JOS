use crate::{
    Error, Result, error::ValidationErrors, interfaces::http::auth::dtos::SignupDto,
    state::AppState,
};
use axum::{
    Extension, Json, Router,
    extract::{Multipart, Path, State},
    response::IntoResponse,
    routing::post,
};
use chrono::Utc;
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;
use validator::Validate;

#[axum::debug_handler]
async fn signup(
    State(app_state): State<Arc<AppState>>,
    Json(new_user_payload): Json<SignupDto>,
) -> Result<Json<String>> {
    new_user_payload
        .validate()
        .map_err(|err| Error::Validation(ValidationErrors::Other(err)))?;

    match app_state
        .user_service
        .create(&new_user_payload.into())
        .await
    {
        Ok(user_id) => Ok(Json(user_id.to_string())),
        Err(err) => Err(err),
    }
}

pub async fn upload_image(
    State(app_state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,

    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filed_name = field.name().unwrap().to_string();

        if filed_name == "image" {
            let data = field.bytes().await.unwrap();

            let img_name = Utc::now().timestamp();

            let mut file = File::create(format!("./public/uploads/{}.png", img_name))
                .await
                .unwrap();
            file.write(&data).await.unwrap();
        } else {
            let data = field.text().await.unwrap();
            tracing::error!("field : {filed_name}, data : {data}");
        }
    }

    "ok"
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/upload-image/{uuid}/image", post(upload_image))
        .with_state(state.clone())
}
