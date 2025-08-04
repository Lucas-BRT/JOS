use jos::{
    Result,
    setup::{launch_server, setup_services},
};

#[tokio::main]
async fn main() -> Result<()> {
    let state = setup_services().await?;

    launch_server(state).await
}
