use utoipa::Modify;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(utoipa::ToSchema)]
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi.components.get_or_insert_default();
        let http_auth = Http::new(HttpAuthScheme::Bearer);
        let security_scheme = SecurityScheme::Http(http_auth);

        components.add_security_scheme("auth", security_scheme)
    }
}

#[derive(utoipa::OpenApi)]
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
        crate::http::handlers::user::get_user_by_id,
        crate::http::handlers::table::create_table,
        crate::http::handlers::table::get_tables,
        crate::http::handlers::table::get_table_details,
        crate::http::handlers::table::update_table,
        crate::http::handlers::table::delete_table,
        crate::http::handlers::session::create_session,
        crate::http::handlers::session::get_sessions,
        crate::http::handlers::session::update_session,
        crate::http::handlers::session::delete_session,
        crate::http::handlers::table_request::create_table_request,
        crate::http::handlers::table_request::get_sent_requests,
        crate::http::handlers::table_request::get_received_requests,
        crate::http::handlers::table_request::accept_request,
        crate::http::handlers::table_request::reject_request,
        crate::http::handlers::table_request::cancel_request,
        crate::http::handlers::game_system::create_game_system,
        crate::http::handlers::game_system::get_game_systems,
        crate::http::handlers::health::health_check
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "tables", description = "RPG table management endpoints"),
        (name = "sessions", description = "Session management endpoints"),
        (name = "table-requests", description = "Table request management endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "game_systems", description = "GameSystems endpoints")
    ),
    info(
        title = "JOS",
        description = "Join Our Session (JOS) - API for managing RPG tables and sessions",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;
