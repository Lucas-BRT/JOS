use crate::{core::state::AppState, interfaces::http::table::dtos::CreateTableDto};
use axum::{Router, routing::post};
use std::{path::Path, sync::Arc};
use tokio::fs;
use uuid::Uuid;

async fn create_table(dto: CreateTableDto) -> Result<String, String> {
    println!("{}", dto.gm_id);

    // save the image on the harddrive

    if let Some(image_file) = dto.image_file {
        let upload_dir = Path::new("../uploads");
        if !upload_dir.exists() {
            if let Err(e) = fs::create_dir_all(upload_dir).await {
                eprintln!("Failed to create upload directory: {}", e);
                return Err("Failed to create upload directory".to_string());
            }
        }

        let filename = format!("{}_{}", Uuid::new_v4(), image_file.filename);
        let filepath = upload_dir.join(filename);

        if let Err(e) = fs::write(&filepath, &image_file.data).await {
            eprintln!("Failed to save file: {}", e);
            return Err("Failed to save image".to_string());
        }

        println!("File saved to: {}", filepath.display());
    }

    Ok("Table created".to_string())
}

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(create_table))
        .with_state(state.clone())
}
