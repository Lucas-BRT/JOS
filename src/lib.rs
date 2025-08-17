pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Re-export commonly used items
pub use core::error::Error;
pub use core::setup::SetupError;

// re-export core modules
pub use core::*;
