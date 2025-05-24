use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;

use crate::application::services::user_service::create;
use crate::application::services::user_service::find_by_username;
use crate::application::services::user_service::get_all;
use crate::application::services::user_service::update;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/", put(update))
        .route("/", post(create))
        .route("/{username}", get(find_by_username))
        .with_state(pool)
}
