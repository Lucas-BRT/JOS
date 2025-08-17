use utoipa::Modify;
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
