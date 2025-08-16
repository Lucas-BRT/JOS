pub mod api_doc;
pub mod auth;
pub mod routes;

pub use api_doc::ApiDoc as OpenApiDoc;
pub use auth::BearerAuth as OpenApiBearerAuth;
pub use routes::routes as OpenApiRoutes;
