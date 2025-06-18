use crate::{
    domain::{table::entity::PlayerExperience, utils::image_file::ImageFile},
    interfaces::http::error::ValidationError,
};
use axum::extract::{FromRequest, Multipart, multipart::Field};
use std::future::Future;
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

impl ImageFile {
    pub async fn from_field(mut field: Field<'_>) -> Result<Self, ValidationError> {
        let content_type = field.content_type().unwrap_or("").to_string();
        if !ALLOWED_IMAGE_TYPES.contains(&content_type.as_str()) {
            return Err(ValidationError::BadRequest {
                reason: "Invalid image type".to_string(),
            });
        }

        let file_name = field.file_name().unwrap_or("upload").to_string();
        let mut data = bytes::BytesMut::new();

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|_| ValidationError::BadRequest {
                reason: "Failed to read image data".to_string(),
            })?
        {
            if (data.len() + chunk.len()) > MAX_IMAGE_SIZE {
                return Err(ValidationError::BadRequest {
                    reason: "Image size exceeds 5Mb limit".to_string(),
                });
            }
            data.extend_from_slice(&chunk);
        }

        Ok(Self {
            filename: file_name,
            content_type,
            data: data.freeze(),
        })
    }
}

async fn parse_unique_field<T, F, Fut>(
    current: &mut Option<T>,
    field_name: &str,
    f: F,
) -> Result<(), ValidationError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, ValidationError>>,
{
    if current.is_some() {
        return Err(ValidationError::DuplicatedFieldError {
            field: field_name.to_string(),
        });
    }

    let value = f().await?;
    *current = Some(value);
    Ok(())
}

#[derive(Debug, Clone, Validate)]
pub struct CreateTableDto {
    pub gm_id: Uuid,
    #[validate(length(
        min = MIN_TITLE_LENGTH,
        max = MAX_TITLE_LENGTH,
        message = "Title must be between 8 and 60 characters"
    ))]
    pub title: String,
    #[validate(length(
        min = MIN_DESCRIPTION_LENGTH,
        max = MAX_DESCRIPTION_LENGTH,
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
    type Rejection = ValidationError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut multipart =
            Multipart::from_request(req, state)
                .await
                .map_err(|_| ValidationError::BadRequest {
                    reason: "Invalid multipart".to_string(),
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
                    parse_unique_field(&mut gm_id, "gm_id", || async {
                        let value = field.text().await.unwrap_or_default();
                        Uuid::parse_str(&value).map_err(|e| ValidationError::InvalidGmId {
                            value: e.to_string(),
                        })
                    })
                    .await?;
                }
                "title" => {
                    parse_unique_field(&mut title, "title", || async {
                        Ok(field.text().await.unwrap_or_default())
                    })
                    .await?;
                }
                "description" => {
                    parse_unique_field(&mut description, "description", || async {
                        Ok(field.text().await.unwrap_or_default())
                    })
                    .await?
                }
                "game_system_id" => {
                    parse_unique_field(&mut game_system_id, "game_system_id", || async {
                        let value = field.text().await.unwrap_or_default();
                        let id = Uuid::parse_str(&value).map_err(|e| {
                            ValidationError::InvalidGameId {
                                value: e.to_string(),
                            }
                        })?;

                        Ok(id)
                    })
                    .await?;
                }
                "is_public" => {
                    parse_unique_field(&mut is_public, "is_public", || async {
                        let value = field.text().await.unwrap_or_default();
                        let is_public =
                            value
                                .parse::<bool>()
                                .map_err(|_| ValidationError::BadRequest {
                                    reason: "is_public must be a boolean value".to_string(),
                                })?;

                        Ok(is_public)
                    })
                    .await?;
                }
                "player_slots" => {
                    parse_unique_field(&mut player_slots, "player_slots", || async {
                        let value = field.text().await.unwrap_or_default();
                        let player_slots =
                            value
                                .parse::<u32>()
                                .map_err(|_| ValidationError::BadRequest {
                                    reason: "player_slots must be an unsigned integer".to_string(),
                                })?;

                        Ok(player_slots)
                    })
                    .await?;
                }
                "recommended_player_experience" => {
                    parse_unique_field(
                        &mut recommended_player_experience,
                        "recommended_player_experience",
                        || async {
                            let value = field.text().await.unwrap_or_default();
                            let recommended_player_experience = match value.to_lowercase().as_str()
                            {
                                "beginner" => PlayerExperience::Beginner,
                                "intermediate" => PlayerExperience::Intermediate,
                                "advanced" => PlayerExperience::Advanced,
                                "expert" => PlayerExperience::Expert,
                                _ => {
                                    return Err(ValidationError::BadRequest {
                                        reason: "invalid recommended player experiene".to_string(),
                                    });
                                }
                            };
                            Ok(recommended_player_experience)
                        },
                    )
                    .await?;
                }
                "image" => {
                    parse_unique_field(&mut image_file, "image", || async {
                        ImageFile::from_field(field).await
                    })
                    .await?;
                }
                _ => {
                    tracing::warn!("ignoring unexpected field: {}", name);
                }
            }
        }

        let dto = CreateTableDto {
            gm_id: gm_id.ok_or(ValidationError::MissingField {
                field: "gm_id".to_string(),
            })?,
            title: title.ok_or(ValidationError::MissingField {
                field: "title".to_string(),
            })?,
            description: description.ok_or(ValidationError::MissingField {
                field: "description".to_string(),
            })?,
            game_system_id: game_system_id.ok_or(ValidationError::MissingField {
                field: "game_system_id".to_string(),
            })?,
            is_public: is_public.ok_or(ValidationError::MissingField {
                field: "is_public".to_string(),
            })?,
            player_slots: player_slots.ok_or(ValidationError::MissingField {
                field: "player_slots".to_string(),
            })?,
            image_file,
            recommended_player_experience,
        };

        dto.validate().map_err(|e| ValidationError::BadRequest {
            reason: format!("Validation error: {}", e),
        })?;

        Ok(dto)
    }
}
