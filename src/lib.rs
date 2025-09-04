pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod shared;

// Re-export commonly used items
pub use shared::{Db, Error, Result};
