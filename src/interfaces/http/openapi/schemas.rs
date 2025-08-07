use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// User schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignupRequest {
    #[schema(example = "john_doe", min_length = 4, max_length = 100)]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123", min_length = 8, max_length = 200)]
    pub password: String,
    #[schema(example = "password123", min_length = 8, max_length = 200)]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123", min_length = 8, max_length = 200)]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserSignupResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

// Table schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct TableResponse {
    pub id: String,
    pub gm_id: String,
    pub title: String,
    pub description: String,
    pub game_system_id: String,
    pub is_public: bool,
    pub max_players: u32,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTableRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub gm_id: String,
    #[schema(example = "Dungeons & Dragons Campaign", min_length = 8, max_length = 60)]
    pub title: String,
    #[schema(example = "A thrilling adventure in the Forgotten Realms...", min_length = 50, max_length = 1000)]
    pub description: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub game_system_id: String,
    #[schema(example = true)]
    pub is_public: bool,
    #[schema(example = 6, minimum = 1, maximum = 20)]
    pub max_players: u32,
    #[schema(example = 6)]
    pub player_slots: u32,
    #[schema(example = 0)]
    pub occupied_slots: u32,
    #[schema(example = "https://example.com/bg-image.jpg")]
    pub bg_image_link: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableTableResponse {
    pub gm_id: String,
    pub title: String,
    pub description: String,
    pub game_system_id: String,
    pub is_public: bool,
    pub max_players: u32,
    pub player_slots: u32,
    pub occupied_slots: u32,
    pub bg_image_link: Option<String>,
}

// Table Request schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct TableRequestResponse {
    pub id: String,
    pub user_id: String,
    pub table_id: String,
    pub message: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTableRequestRequest {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174001")]
    pub table_id: String,
    #[schema(example = "I would like to join this table!", max_length = 500)]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTableRequestRequest {
    #[schema(example = "approved")]
    pub status: String,
}

// Error schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "error_message")]
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ValidationErrorResponse {
    #[schema(example = "validation")]
    pub validation: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordMismatchErrorResponse {
    pub password_confirmation: Vec<String>,
}

// Success response schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    #[schema(example = "Operation completed successfully")]
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IdResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,
}

// File upload schemas
#[derive(Debug, Serialize, ToSchema)]
pub struct FileUploadResponse {
    #[schema(example = "image_123e4567-e89b-12d3-a456-426614174000_1234567890.png")]
    pub filename: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FileUploadErrorResponse {
    #[schema(example = "image too large")]
    pub error: String,
    #[schema(example = "maximum allowed size is 5MB")]
    pub details: String,
}
