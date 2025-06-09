use crate::{Error, Result, core::error::ApplicationSetupError};
use std::{net::SocketAddr, num::ParseIntError, str::FromStr};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub addr: SocketAddr,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").map_err(|e| {
            Error::ApplicationSetup(ApplicationSetupError::FailedToGetEnvironmentVariable(
                e.to_string(),
            ))
        })?;

        let server_port: u32 = std::env::var("PORT")
            .map_err(|e| ApplicationSetupError::FailedToGetEnvironmentVariable(e.to_string()))?
            .parse()
            .map_err(|e: ParseIntError| ApplicationSetupError::FailedToParsePort(e.to_string()))?;

        let addr = SocketAddr::from_str(format!("127.0.0.1:{}", server_port).as_str())
            .map_err(|err| ApplicationSetupError::FailedToSetupServerAddress(err.to_string()))?;

        let jwt_secret = std::env::var("JWT_SECRET").map_err(|e| {
            Error::ApplicationSetup(ApplicationSetupError::FailedToGetEnvironmentVariable(
                e.to_string(),
            ))
        })?;

        Ok(Self {
            database_url,
            addr,
            jwt_secret,
        })
    }
}
