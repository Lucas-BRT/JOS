use crate::{
    Error,
    domain::{table::entity::PlayerExperience, utils::image_file::ImageFile},
    interfaces::http::error::ValidationError,
};
use axum::extract::{FromRequest, Multipart};
use uuid::Uuid;
use validator::Validate;

const MIN_TITLE_LENGTH: u64 = 8;
const MAX_TITLE_LENGTH: u64 = 60;
const MIN_DESCRIPTION_LENGTH: u64 = 50;
const MAX_DESCRIPTION_LENGTH: u64 = 1000;
const MIN_PLAYER_SLOTS: u32 = 1;
const MAX_PLAYER_SLOTS: u32 = 30;
const ALLOWED_IMAGE_TYPES: [&str; 2] = ["image/jpeg", "image/png"];
const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5 MB

#[derive(Debug, Clone, Validate)]
pub struct CreateTableDto {
    pub gm_id: Uuid,
    #[validate(length(
        min = "MIN_TITLE_LENGTH",
        max = "MAX_TITLE_LENGTH",
        message = "Title must be between 8 and 60 characters"
    ))]
    pub title: String,
    #[validate(length(
        min = "MIN_DESCRIPTION_LENGTH",
        max = "MAX_DESCRIPTION_LENGTH",
        message = "Description must be between 50 and 1000 characters"
    ))]
    pub description: String,
    pub game_system_id: Uuid,
    pub is_public: bool,
    #[validate(range(
        min = "MIN_PLAYER_SLOTS",
        max = "MAX_PLAYER_SLOTS",
        message = "Player slots must be between 1 and 30"
    ))]
    pub player_slots: u32,
    pub image_file: Option<ImageFile>,
    pub recommended_player_experience: Option<PlayerExperience>,
}

impl<S> FromRequest<S> for CreateTableDto
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await.map_err(|_| {
            Error::Validation(ValidationError::BadRequest("Invalid multipart".to_string()))
        })?;

        let mut gm_id = None;
        let mut title = None;
        let mut description = None;
        let mut game_system_id = None;
        let mut is_public = None;
        let mut player_slots = None;
        let mut image_file = None;
        let mut recommended_player_experience = None;

        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();

            match name.as_str() {
                "gm_id" => {
                    let value = field.text().await.unwrap_or_default();
                    gm_id = Some(Uuid::parse_str(&value).map_err(|e| {
                        Error::Validation(ValidationError::InvalidGmId(e.to_string()))
                    })?);
                }
                "title" => {
                    title = Some(field.text().await.unwrap_or_default());
                }
                "description" => {
                    description = Some(field.text().await.unwrap_or_default());
                }
                "game_system_id" => {
                    let value = field.text().await.unwrap_or_default();
                    game_system_id = Some(Uuid::parse_str(&value).map_err(|e| {
                        Error::Validation(ValidationError::InvalidGameId(e.to_string()))
                    })?);
                }
                "is_public" => {
                    let value = field.text().await.unwrap_or_default();
                    is_public = Some(value.parse::<bool>().map_err(|_| {
                        Error::Validation(ValidationError::BadRequest(
                            "is_public must be true or false".to_string(),
                        ))
                    })?);
                }
                "player_slots" => {
                    let value = field.text().await.unwrap_or_default();
                    player_slots = Some(value.parse::<u32>().map_err(|_| {
                        Error::Validation(ValidationError::BadRequest(
                            "player_slots must be an integer".to_string(),
                        ))
                    })?);
                }
                "recommended_player_experience" => {
                    let value = field.text().await.unwrap_or_default();
                    recommended_player_experience = match value.to_lowercase().as_str() {
                        "beginner" => Some(PlayerExperience::Beginner),
                        "intermediate" => Some(PlayerExperience::Intermediate),
                        "advanced" => Some(PlayerExperience::Advanced),
                        "expert" => Some(PlayerExperience::Expert),
                        _ => None,
                    };
                }
                "image" => {
                    let content_type = field.content_type().unwrap_or("").to_string();
                    if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
                        return Err(Error::Validation(ValidationError::BadRequest(
                            "Invalid image type".to_string(),
                        )));
                    }

                    let file_name = field.file_name().unwrap_or("upload").to_string();

                    let mut data = bytes::BytesMut::new();
                    let mut field_stream = field;

                    while let Some(chunk) = field_stream.chunk().await.map_err(|_| {
                        Error::Validation(ValidationError::BadRequest(
                            "Failed to read image data".to_string(),
                        ))
                    })? {
                        tracing::info!("Received chunk of size {}", chunk.len());
                        if (data.len() + chunk.len()) > MAX_IMAGE_SIZE {
                            return Err(Error::Validation(ValidationError::BadRequest(
                                "Image size exceeds 5MB limit".to_string(),
                            )));
                        }
                        data.extend_from_slice(&chunk);
                    }

                    if !data.is_empty() {
                        image_file = Some(ImageFile {
                            filename: file_name,
                            content_type,
                            data: data.freeze(),
                        });
                    }
                }
                _ => {}
            }
        }

        let dto = CreateTableDto {
            gm_id: gm_id.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("gm_id".to_string()))
            })?,
            title: title.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("title".to_string()))
            })?,
            description: description.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("description".to_string()))
            })?,
            game_system_id: game_system_id.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("game_system_id".to_string()))
            })?,
            is_public: is_public.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("is_public".to_string()))
            })?,
            player_slots: player_slots.ok_or_else(|| {
                Error::Validation(ValidationError::MissingField("player_slots".to_string()))
            })?,
            image_file,
            recommended_player_experience,
        };

        dto.validate().map_err(|e| {
            Error::Validation(ValidationError::BadRequest(format!(
                "Validation error: {}",
                e
            )))
        })?;

        Ok(dto)
    }
}
