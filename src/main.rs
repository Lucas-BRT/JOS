use infrastructure::setup::server::Server;
use jos::infrastructure::{
    config::AppConfig,
    setup::{database::setup_database, logging::init_logging},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    let config = AppConfig::from_env().expect("failed to load configuration");
    let database = setup_database(&config.database_url)
        .await
        .expect("failed to setup database");

    let server = Server::new()
        .setup_services(&database, &config)
        .await?
        .launch()
        .await;

    Ok(())
}
