use utoipa::Modify;
use utoipa::OpenApi as OpenApiSchema;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(utoipa::ToSchema)]
pub struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi.components.get_or_insert_default();
        let http_auth = Http::new(HttpAuthScheme::Bearer);
        let security_scheme = SecurityScheme::Http(http_auth);

        components.add_security_scheme("bearer_auth", security_scheme)
    }
}

#[derive(OpenApiSchema)]
#[openapi(
    paths(
        crate::http::handlers::auth::login,
        crate::http::handlers::auth::register,
        crate::http::handlers::auth::refresh,
        crate::http::handlers::auth::logout,
        crate::http::handlers::auth::me,
        crate::http::handlers::auth::update_profile,
        crate::http::handlers::auth::change_password,
        crate::http::handlers::auth::delete_account,
        crate::http::handlers::table::create_table,
        crate::http::handlers::table::get_tables,
        crate::http::handlers::table::get_table_details,
        crate::http::handlers::table::update_table,
        crate::http::handlers::table::delete_table,
        crate::http::handlers::session::create_session,
        crate::http::handlers::session::get_sessions,
        crate::http::handlers::session::get_session_details,
        crate::http::handlers::session::update_session,
        crate::http::handlers::session::delete_session,
        crate::http::handlers::table_request::create_table_request,
        crate::http::handlers::table_request::get_sent_requests,
        crate::http::handlers::table_request::get_received_requests,
        crate::http::handlers::table_request::accept_request,
        crate::http::handlers::table_request::reject_request,
        crate::http::handlers::table_request::cancel_request,
        crate::http::handlers::search::search,
        crate::http::handlers::health::health_check
    ),
    components(
        schemas(
            crate::http::dtos::auth::LoginRequest,
            crate::http::dtos::auth::LoginResponse,
            crate::http::dtos::auth::RegisterRequest,
            crate::http::dtos::auth::RegisterResponse,
            crate::http::dtos::auth::RefreshTokenRequest,
            crate::http::dtos::auth::RefreshTokenResponse,
            crate::http::dtos::auth::LogoutResponse,
            crate::http::dtos::auth::UserResponse,
            crate::http::dtos::user::UpdateProfileRequest,
            crate::http::dtos::user::UpdateProfileResponse,
            crate::http::dtos::user::ChangePasswordRequest,
            crate::http::dtos::user::ChangePasswordResponse,
            crate::http::dtos::user::DeleteAccountRequest,
            crate::http::dtos::user::DeleteAccountResponse,
            crate::http::dtos::table::CreateTableRequest,
            crate::http::dtos::table::UpdateTableRequest,
            crate::http::dtos::table::DeleteTableResponse,
            crate::http::dtos::session::CreateSessionRequest,
            crate::http::dtos::session::UpdateSessionRequest,
            crate::http::dtos::session::DeleteSessionResponse,
            crate::http::dtos::request::CreateTableRequestRequest,
            crate::http::dtos::request::TableRequestResponse,
            crate::http::dtos::request::SentRequestItem,
            crate::http::dtos::request::ReceivedRequestItem,
            crate::http::dtos::request::AcceptRequestResponse,
            crate::http::dtos::request::RejectRequestResponse,
            crate::http::dtos::request::CancelRequestResponse,
            crate::http::dtos::common::SearchTablesQuery,
            crate::http::handlers::search::SearchQuery,
            crate::http::handlers::search::SearchResult,
            crate::http::handlers::search::SearchResponse,
            crate::http::dtos::common::ErrorResponse
        )
    ),
    security(("bearer_auth" = [])),
    modifiers(&BearerAuth),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "tables", description = "RPG table management endpoints"),
        (name = "sessions", description = "Session management endpoints"),
        (name = "table-requests", description = "Table request management endpoints"),
        (name = "search", description = "Search endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "JOS API",
        description = "Join Our Session (JOS) - API for managing RPG tables and sessions",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;
