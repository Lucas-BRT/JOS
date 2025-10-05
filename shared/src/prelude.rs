use crate::error::Error;
use chrono::{DateTime, Utc};

pub type Result<T> = std::result::Result<T, Error>;

pub type Date = DateTime<Utc>;
