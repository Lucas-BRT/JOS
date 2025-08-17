use jos::{
    Error, Result,
    setup::{launch_server, setup_services},
};

#[tokio::main]
async fn main() -> Result<()> {
    match setup_services().await {
        Ok(state) => launch_server(state).await,
        Err(Error::Setup(setup_error)) => {
            eprintln!("\n{}", setup_error.user_friendly_message());
            std::process::exit(1);
        }
        Err(other_error) => {
            eprintln!("\n❌ Application error: {other_error}");
            std::process::exit(1);
        }
    }
}
