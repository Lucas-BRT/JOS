use axum::{Router};
use utoipa::openapi::security::{Http, SecurityScheme, HttpAuthScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use crate::interfaces::http::auth::dtos::*;
use crate::interfaces::http::table::dtos::*;
use crate::interfaces::http::table_request::dtos::*;

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
            MeResponse,
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

/// JWT Bearer token security scheme
#[derive(utoipa::ToSchema)]
pub struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_default();
        let http_auth = Http::new(HttpAuthScheme::Bearer);
        let security_scheme = SecurityScheme::Http(http_auth);

        components.add_security_scheme("bearer_auth", security_scheme)
    }
}



pub fn routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}


