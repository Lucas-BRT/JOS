use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// User signup request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupDto {
    /// User's full name (4-100 characters)
    #[schema(min_length = 4, max_length = 100)]
    pub name: String,
    /// User's email address
    #[schema(format = "email")]
    pub email: String,
    /// User's password (8-200 characters)
    #[schema(min_length = 8, max_length = 200)]
    pub password: String,
    /// Password confirmation (must match password)
    pub confirm_password: String,
}

/// User login request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    /// User's email address
    #[schema(format = "email")]
    pub email: String,
    /// User's password
    pub password: String,
}

/// User signup response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserSignupResponse {
    /// User's unique identifier
    pub id: String,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
}

/// Login response
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// JWT token for authentication
    pub token: String,
}

/// User information response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    /// User's unique identifier
    pub id: String,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
    /// User's role
    pub role: String,
    /// Account creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: Option<String>,
}

/// Current user information
#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    /// User's unique identifier
    pub id: String,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
    /// User's role
    pub role: String,
    /// Account creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: Option<String>,
}

/// Password requirements response
#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordRequirementsResponse {
    /// Password validation requirements
    pub requirements: Vec<String>,
}

/// Create table request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTableDto {
    /// Table title (8-60 characters)
    #[schema(min_length = 8, max_length = 60)]
    pub title: String,
    /// Table description (50-1000 characters)
    #[schema(min_length = 50, max_length = 1000)]
    pub description: String,
    /// Game system identifier
    #[schema(format = "uuid")]
    pub game_system_id: String,
    /// Whether the table is public
    pub is_public: bool,
    /// Maximum number of players (1-20)
    #[schema(minimum = 1, maximum = 20)]
    pub max_players: i32,
}

/// Available table response
#[derive(Debug, Serialize, ToSchema)]
pub struct AvailableTableResponse {
    /// Table's unique identifier
    pub id: String,
    /// Table title
    pub title: String,
    /// Table description
    pub description: String,
    /// Game system identifier
    pub game_system_id: String,
    /// Whether the table is public
    pub is_public: bool,
    /// Maximum number of players
    pub max_players: i32,
    /// Current number of players
    pub current_players: i32,
    /// Table creation timestamp
    pub created_at: String,
}

/// Create table request DTO
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTableRequestDto {
    /// Table identifier
    #[schema(format = "uuid")]
    pub table_id: String,
    /// Request message (max 500 characters)
    #[schema(max_length = 500)]
    pub message: String,
}

/// Table request response
#[derive(Debug, Serialize, ToSchema)]
pub struct TableRequestResponse {
    /// Request's unique identifier
    pub id: String,
    /// Table identifier
    pub table_id: String,
    /// User identifier
    pub user_id: String,
    /// Request message
    pub message: String,
    /// Request status
    pub status: String,
    /// Request creation timestamp
    pub created_at: String,
}

/// Update table request DTO
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTableRequestDto {
    /// New status for the request
    pub status: String,
}

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Current timestamp
    pub timestamp: String,
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
}

/// Error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error details (optional)
    pub details: Option<String>,
}

/// Validation error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ValidationErrorResponse {
    /// Error message
    pub error: String,
    /// Field validation errors
    pub field_errors: Vec<FieldError>,
}

/// Field validation error
#[derive(Debug, Serialize, ToSchema)]
pub struct FieldError {
    /// Field name
    pub field: String,
    /// Error message
    pub message: String,
}
