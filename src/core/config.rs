use crate::{Result, setup::SetupError, Error};
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
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))?;

        let server_port: u32 = std::env::var("PORT")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))?
            .parse()
            .map_err(|e: ParseIntError| SetupError::FailedToParsePort(e.to_string()))?;

        // Validate port range
        if server_port < 1024 || server_port > 65535 {
            return Err(Error::Setup(SetupError::InvalidConfiguration(
                format!("Port {} is outside valid range (1024-65535)", server_port)
            )));
        }

        let addr = SocketAddr::from_str(format!("127.0.0.1:{server_port}").as_str())
            .map_err(|err| SetupError::FailedToSetupServerAddress(err.to_string()))?;

        let jwt_secret = JWT_SECRET.deref().clone();

        // Validate JWT secret
        if jwt_secret.len() < 32 {
            tracing::warn!("⚠️  JWT_SECRET is shorter than recommended (32+ characters)");
        }

        // Validate database URL format
        if !database_url.starts_with("postgres://") && !database_url.starts_with("postgresql://") {
            return Err(Error::Setup(SetupError::InvalidConfiguration(
                "DATABASE_URL must start with 'postgres://' or 'postgresql://'".to_string()
            )));
        }

        Ok(Self {
            database_url,
            addr,
            jwt_secret,
            jwt_expiration_duration: JWT_EXPIRATION_DURATION,
        })
    }

    /// Returns a human-readable description of the configuration
    pub fn describe(&self) -> String {
        format!(
            "Server: {}, Database: {}, JWT Expiration: {} days",
            self.addr,
            self.database_url.split('@').last().unwrap_or("unknown"),
            self.jwt_expiration_duration.num_days()
        )
    }
}
