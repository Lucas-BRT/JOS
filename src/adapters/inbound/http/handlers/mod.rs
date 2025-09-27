pub mod auth;
pub mod docs;
pub mod health;
pub mod search;
pub mod session;
pub mod table;
pub mod table_request;
pub mod user;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest(
            "/v1",
            Router::new()
                .nest("/auth", auth_routes(app_state.clone()))
                .nest("/users", user_routes(app_state.clone()))
                .nest("/tables", table_routes(app_state.clone()))
                .nest("/sessions", session_routes(app_state.clone()))
                .nest("/requests", table_request_routes(app_state.clone()))
                .nest("/search", search_routes(app_state.clone())),
        )
        .merge(OpenApiRoutes())
        .layer(cors)
}
