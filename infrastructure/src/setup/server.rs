use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::state::AppState;
use shared::Result;
use shared::error::Error;
use shared::error::SetupError;

pub fn setup_server(app_state: AppState) -> Router {
    Router::new().with_state(app_state)
}

pub async fn launch_server(router: Router, app_state: &AppState) -> Result<()> {
    info!("🚀 Launching HTTP server...");

    let listener = TcpListener::bind(&app_state.config.addr)
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
