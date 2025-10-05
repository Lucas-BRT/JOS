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
        crate::adapters::inbound::http::handlers::auth::login,
        crate::adapters::inbound::http::handlers::auth::register,
        crate::adapters::inbound::http::handlers::auth::refresh,
        crate::adapters::inbound::http::handlers::auth::logout,
        crate::adapters::inbound::http::handlers::auth::me,
        crate::adapters::inbound::http::handlers::user::update_profile,
        crate::adapters::inbound::http::handlers::user::change_password,
        crate::adapters::inbound::http::handlers::user::delete_account,
        crate::adapters::inbound::http::handlers::table::create_table,
        crate::adapters::inbound::http::handlers::table::get_tables,
        crate::adapters::inbound::http::handlers::table::get_table_details,
        crate::adapters::inbound::http::handlers::table::update_table,
        crate::adapters::inbound::http::handlers::table::delete_table,
        crate::adapters::inbound::http::handlers::session::create_session,
        crate::adapters::inbound::http::handlers::session::get_sessions,
        crate::adapters::inbound::http::handlers::session::get_session_details,
        crate::adapters::inbound::http::handlers::session::update_session,
        crate::adapters::inbound::http::handlers::session::delete_session,
        crate::adapters::inbound::http::handlers::table_request::create_table_request,
        crate::adapters::inbound::http::handlers::table_request::get_sent_requests,
        crate::adapters::inbound::http::handlers::table_request::get_received_requests,
        crate::adapters::inbound::http::handlers::table_request::accept_request,
        crate::adapters::inbound::http::handlers::table_request::reject_request,
        crate::adapters::inbound::http::handlers::table_request::cancel_request,
        crate::adapters::inbound::http::handlers::search::search,
        crate::adapters::inbound::http::handlers::health::health_check
    ),
    components(
        schemas(
            crate::dtos::auth::LoginRequest,
            crate::dtos::auth::LoginResponse,
            crate::dtos::auth::RegisterRequest,
            crate::dtos::auth::RegisterResponse,
            crate::dtos::auth::RefreshTokenRequest,
            crate::dtos::auth::RefreshTokenResponse,
            crate::dtos::auth::LogoutResponse,
            crate::dtos::auth::UserResponse,
            crate::dtos::user::UpdateProfileRequest,
            crate::dtos::user::UpdateProfileResponse,
            crate::dtos::user::ChangePasswordRequest,
            crate::dtos::user::ChangePasswordResponse,
            crate::dtos::user::DeleteAccountRequest,
            crate::dtos::user::DeleteAccountResponse,
            crate::dtos::table::CreateTableRequest,
            crate::dtos::table::UpdateTableRequest,
            crate::dtos::table::DeleteTableResponse,
            crate::dtos::session::CreateSessionRequest,
            crate::dtos::session::UpdateSessionRequest,
            crate::dtos::session::DeleteSessionResponse,
            crate::dtos::request::CreateTableRequestRequest,
            crate::dtos::request::TableRequestResponse,
            crate::dtos::request::SentRequestItem,
            crate::dtos::request::ReceivedRequestItem,
            crate::dtos::request::AcceptRequestResponse,
            crate::dtos::request::RejectRequestResponse,
            crate::dtos::request::CancelRequestResponse,
            crate::dtos::common::SearchTablesQuery,
            crate::adapters::inbound::http::handlers::search::SearchQuery,
            crate::adapters::inbound::http::handlers::search::SearchResult,
            crate::adapters::inbound::http::handlers::search::SearchResponse,
            crate::dtos::common::ErrorResponse
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
