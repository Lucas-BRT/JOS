use std::sync::Arc;
use tracing::info;

use crate::{Error, Result};
use crate::core::state::AppState;
use crate::core::setup::SetupError;
use crate::interfaces::http::create_router;

pub async fn launch_server(state: Arc<AppState>) -> Result<()> {
    info!("🚀 Launching HTTP server...");

    let listener = tokio::net::TcpListener::bind(&state.config.addr)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToBindAddress(err.to_string())))?;

    let local_addr = listener
        .local_addr()
        .expect("failed to get server addr");

    info!("✅ Server bound to: {}", local_addr);
    info!("🌐 API documentation available at: http://{}/docs", local_addr);
    info!("🔍 Health check available at: http://{}/health", local_addr);

    let router = create_router(state);

    axum::serve(listener, router)
        .await
        .map_err(|err| Error::Setup(SetupError::FailedToLaunchServer(err.to_string())))
}


