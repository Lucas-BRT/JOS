use crate::interfaces::http::{
    auth::dtos::{LoginDto, SignupDto, UserSignupResponse},
    user::dtos::MeResponse,
    table::dtos::CreateTableDto,
    table_request::dtos::{CreateTableRequestDto, UpdateTableRequestDto, TableRequestResponse},
    openapi::{schemas::*, tags::*},
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "JOS API",
        description = "Join Our Session (JOS) - API for managing RPG tables and sessions",
        version = "1.0.0",
        contact(
            name = "JOS Team",
            email = "contact@jos.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000/v1", description = "Development server"),
        (url = "https://api.jos.com/v1", description = "Production server")
    ),
    components(
        schemas(
            // User schemas
            UserResponse,
            SignupDto,
            LoginDto,
            LoginResponse,
            UserSignupResponse,
            MeResponse,
            
            // Table schemas
            TableResponse,
            CreateTableDto,
            AvailableTableResponse,
            
            // Table Request schemas
            TableRequestResponse,
            CreateTableRequestDto,
            UpdateTableRequestDto,
            
            // Error schemas
            ErrorResponse,
            ValidationErrorResponse,
            PasswordMismatchErrorResponse,
            
            // Success schemas
            SuccessResponse,
            IdResponse,
            
            // File upload schemas
            FileUploadResponse,
            FileUploadErrorResponse
        )
    ),
    tags(
        (name = AUTH_TAG, description = "Authentication and authorization endpoints"),
        (name = USER_TAG, description = "User management endpoints"),
        (name = TABLE_TAG, description = "RPG table management endpoints"),
        (name = TABLE_REQUEST_TAG, description = "Table request management endpoints")
    )
)]
pub struct ApiDoc;
