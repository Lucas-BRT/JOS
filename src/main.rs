#![allow(incomplete_features)]

mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod prelude;
mod utils;

use core::{
    error::AppError,
    setup::{launch_server, setup_services},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let state = setup_services().await?;

    launch_server(state).await
}
