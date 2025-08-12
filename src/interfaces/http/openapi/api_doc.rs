use crate::interfaces::http::{auth::dtos::*, user::dtos::MeResponse};
use crate::interfaces::http::table::dtos::*;
use crate::interfaces::http::table_request::dtos::*;
use crate::interfaces::http::openapi::auth::BearerAuth;
use utoipa::{OpenApi};


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::health::health_check,

        crate::interfaces::http::auth::routes::signup,
        crate::interfaces::http::auth::routes::login,
        crate::interfaces::http::auth::routes::get_password_requirements,

        crate::interfaces::http::user::routes::me,
        crate::interfaces::http::user::routes::get_user_by_id,

        crate::interfaces::http::table::routes::create_table,
        crate::interfaces::http::table::routes::get_available_tables,
        crate::interfaces::http::table::routes::delete_table,

        crate::interfaces::http::table_request::routes::get_table_requests_by_table_id,
        crate::interfaces::http::table_request::routes::create_table_request,
        crate::interfaces::http::table_request::routes::get_table_requests,
        crate::interfaces::http::table_request::routes::update_table_request,
        crate::interfaces::http::table_request::routes::delete_table_request,
    ),
    components(
        schemas(
            SignupDto,
            LoginDto,
            UserSignupResponse,
            MeResponse,
            UserResponse,
            CreateTableDto,
            AvaliableTableResponse,
            CreateTableRequestDto,
            TableRequestResponse,
            UpdateTableRequestDto            
        )
    ),
    modifiers(&BearerAuth),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "tables", description = "RPG table management endpoints"),
        (name = "table-requests", description = "Table request management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "JOS API",
        description = "Join Our Session (JOS) - API for managing RPG tables and sessions",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;