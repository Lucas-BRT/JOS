use crate::{Error, Result, setup::SetupError};
use chrono::Duration;
use std::ops::Deref;
use std::sync::LazyLock;
use std::{net::SocketAddr, num::ParseIntError, str::FromStr};

const JWT_EXPIRATION_DURATION: Duration = Duration::days(1);

pub static JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set")
});

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub addr: SocketAddr,
    pub jwt_secret: String,
    pub jwt_expiration_duration: Duration,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| Error::Setup(SetupError::FailedToGetEnvironmentVariable(e.to_string())))?;

        let server_port: u32 = std::env::var("PORT")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))?
            .parse()
            .map_err(|e: ParseIntError| SetupError::FailedToParsePort(e.to_string()))?;

        let addr = SocketAddr::from_str(format!("127.0.0.1:{}", server_port).as_str())
            .map_err(|err| SetupError::FailedToSetupServerAddress(err.to_string()))?;

        let jwt_secret = JWT_SECRET.deref().clone();

        Ok(Self {
            database_url,
            addr,
            jwt_secret,
            jwt_expiration_duration: JWT_EXPIRATION_DURATION,
        })
    }
}
