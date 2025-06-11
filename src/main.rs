#![allow(incomplete_features)]

mod application;
mod core;
mod domain;
mod infrastructure;
mod interfaces;
mod utils;

use core::setup::{launch_server, setup_services};
pub use core::*;

#[tokio::main]
async fn main() -> Result<()> {
    let state = setup_services().await?;

    launch_server(state).await
}
