pub mod auth;
pub mod error;
pub mod openapi;
pub mod routers;
pub mod table;
pub mod table_request;
pub mod user;
pub mod health;

pub use routers::create_router;
