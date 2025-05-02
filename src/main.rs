mod config;
mod error;
mod model;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenvy::dotenv().ok();
    config::Config::from_env()?;

    Ok(())
}
