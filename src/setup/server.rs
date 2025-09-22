use crate::{Error, Result, infrastructure::SetupError};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

pub async fn launch_server(router: &Router) -> Result<()> {
    info!("🚀 Launching HTTP server...");

    let listener = TcpListener::bind(&router.config.addr)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToBindAddress(err.to_string())))?;

    let local_addr = listener.local_addr().expect("failed to get server addr");

    info!("✅ Server bound to: {}", local_addr);
    info!(
        "🌐 API documentation available at: http://{}/docs",
        local_addr
    );
    info!("🔍 Health check available at: http://{}/health", local_addr);

    axum::serve(listener, router)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToLaunchServer(err.to_string())))
}
