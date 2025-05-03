use crate::error::Error;
use std::{net::SocketAddr, str::FromStr};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let db_url = std::env::var("DATABASE_URL").map_err(|err| {
            Error::ApplicationSetup(format!("failed to get DATABASE_URL from env: {}", err))
        })?;

        let server_port: u32 = std::env::var("PORT")
            .map_err(|err| {
                Error::ApplicationSetup(format!("failed to get PORT from env: {}", err))
            })?
            .parse()
            .map_err(|err| {
                Error::ApplicationSetup(format!("failed to parse PORT to u32: {}", err))
            })?;

        let server_addr = SocketAddr::from_str(format!("127.0.0.1:{}", server_port).as_str())
            .map_err(|err| {
                Error::ApplicationSetup(format!("failed to setup server address: {}", err))
            })?;

        Ok(Self {
            database_url: db_url,
            addr: server_addr,
        })
    }
}
