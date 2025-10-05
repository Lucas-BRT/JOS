use jos::adapters::inbound::http::handlers::create_router;
use jos::infrastructure::{
    config::AppConfig,
    setup::{database::setup_database, launch_server, logging::init_logging},
    state::setup_app_state,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    init_logging();
    let config = AppConfig::from_env().expect("failed to load configuration");
    let database = setup_database(&config.database_url)
        .await
        .expect("failed to setup database");

    let app_state = setup_app_state(&database, &config)
        .await
        .expect("failed to setup app state");

    let app_state_arc = Arc::new(app_state);
    let server = create_router(app_state_arc.clone());

    launch_server(server, &app_state_arc)
        .await
        .expect("failed to launch server");
}
