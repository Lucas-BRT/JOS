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
