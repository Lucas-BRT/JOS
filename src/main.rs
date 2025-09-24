use jos::{
    Error, Result,
    infrastructure::setup::{launch_server, setup_services},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup services and create router
    match setup_services().await {
        Ok((router, app_state)) => launch_server(router, app_state).await,
        Err(Error::Setup(setup_error)) => {
            eprintln!("\n❌ Setup error: {setup_error}");
            std::process::exit(1);
        }
        Err(other_error) => {
            eprintln!("\n❌ Application error: {other_error}");
            std::process::exit(1);
        }
    }
}
