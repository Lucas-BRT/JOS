use jos::infrastructure::{
    config::AppConfig,
    setup::{database::setup_database, launch_server, logging::init_logging},
    state::setup_app_state,
};

#[tokio::main]
async fn main() {
    init_logging();
    let config = AppConfig::from_env().expect("failed to load configuration");
    let database = setup_database(&config.database_url)
        .await
        .expect("failed to setup database");

    let app_state = setup_app_state(&database)
        .await
        .expect("failed to setup app state");

    let server = create_router

        .with_state(app_state.clone()).;

    launch_server(server, &app_state)
        .await
        .expect("failed to launch server");
}
