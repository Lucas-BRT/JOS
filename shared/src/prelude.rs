use chrono::{DateTime, Utc};
use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub type Date = DateTime<Utc>;
