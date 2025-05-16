use crate::api::handlers::users;
use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(users::get_users::handle))
        .route("/", put(users::update_users::handle))
        .route("/", post(users::create_user::handle))
        .route("/{username}", get(users::get_user_by_username::handle))
        .with_state(pool)
}
