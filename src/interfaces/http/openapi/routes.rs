use axum::{Router, Json};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::http::openapi::schemas::*;

/// JOS API - Join Our Session
/// 
/// API for managing RPG tables and sessions
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::routers::health_check,
        crate::interfaces::http::auth::routes::signup,
        crate::interfaces::http::auth::routes::login,
        crate::interfaces::http::auth::routes::get_password_requirements,
        crate::interfaces::http::user::routes::me,
        crate::interfaces::http::table::routes::create_table,
        crate::interfaces::http::table::routes::get_available_tables,
        crate::interfaces::http::table_request::routes::create_table_request,
        crate::interfaces::http::table_request::routes::get_table_requests,
        crate::interfaces::http::table_request::routes::get_table_request_by_id,
        crate::interfaces::http::table_request::routes::update_table_request,
        crate::interfaces::http::table_request::routes::delete_table_request
    ),
    components(
        schemas(
            SignupDto,
            LoginDto,
            UserSignupResponse,
            LoginResponse,
            UserResponse,
            MeResponse,
            PasswordRequirementsResponse,
            CreateTableDto,
            AvailableTableResponse,
            CreateTableRequestDto,
            TableRequestResponse,
            UpdateTableRequestDto,
            HealthResponse,
            ErrorResponse,
            ValidationErrorResponse,
            FieldError
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "tables", description = "RPG table management endpoints"),
        (name = "table-requests", description = "Table request management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    servers(
        (url = "http://localhost:3000", description = "Development server"),
        (url = "https://api.jos.com", description = "Production server")
    ),
    info(
        title = "JOS API",
        description = "Join Our Session (JOS) - API for managing RPG tables and sessions",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;

/// JWT Bearer token security scheme
#[derive(utoipa::ToSchema)]
pub struct BearerAuth;

pub fn routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}


