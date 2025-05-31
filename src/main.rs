#![allow(incomplete_features)]

use core::{
    error::{AppError, ApplicationSetupError},
    setup::{launch_server, setup_services},
};

use interfaces::http::create_router;

mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod prelude;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let state = setup_services().await?;

    launch_server(state).await
}
