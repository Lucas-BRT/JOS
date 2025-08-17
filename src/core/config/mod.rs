mod constants;
mod enviroment;
mod jwt;

use crate::{
    Result,
    config::{constants::DEFAULT_JWT_SECRET, enviroment::Environment},
    setup::SetupError,
};
use chrono::Duration;
use constants::{DEFAULT_HOST, DEFAULT_JWT_EXPIRATION_DURATION, MIN_JWT_SECRET_LEN};
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::str::FromStr;
use tracing::{info, warn};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub addr: SocketAddr,
    pub jwt_secret: String,
    pub jwt_expiration_duration: Duration,
    pub environment: Environment,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))?;

        let server_port: u32 = std::env::var("PORT")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))?
            .parse()
            .map_err(|e: ParseIntError| SetupError::FailedToParsePort(e.to_string()))?;

        let addr = SocketAddr::from_str(format!("{DEFAULT_HOST}:{server_port}").as_str())
            .map_err(|err| SetupError::FailedToSetupServerAddress(err.to_string()))?;

        let environment = std::env::var("ENVIRONMENT")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))
            .map(|value| {
                if value == "production" {
                    Environment::Production
                } else {
                    Environment::Development
                }
            })
            .unwrap_or_default();

        let mut jwt_expiration_duration = std::env::var("JWT_EXPIRATION_DURATION")
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .map(Duration::days)
            .unwrap_or(DEFAULT_JWT_EXPIRATION_DURATION);

        if jwt_expiration_duration.num_days() < 1 {
            tracing::warn!("⚠️  JWT_EXPIRATION_DURATION is less than 1 day, setting to 1 day");
            jwt_expiration_duration = Duration::days(1);
        }

        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| {
                tracing::warn!(
                    "⚠️  JWT_SECRET environment variable is not set, using default value"
                );
            })
            .unwrap_or(DEFAULT_JWT_SECRET.to_string());

        if jwt_secret.len() < MIN_JWT_SECRET_LEN {
            tracing::warn!(
                "⚠️  JWT_SECRET is shorter than recommended ({}+ characters)",
                MIN_JWT_SECRET_LEN
            );
        }

        Ok(Self {
            database_url,
            addr,
            jwt_secret,
            jwt_expiration_duration,
            environment,
        })
    }

    pub fn describe(&self) -> String {
        format!(
            "Server: {}, Database: {}, JWT Expiration: {} days",
            self.addr,
            self.database_url.split('@').next_back().unwrap_or("unknown"),
            self.jwt_expiration_duration.num_days()
        )
    }

    pub fn validate_config(&self) -> Result<()> {
        if self.jwt_secret.len() < MIN_JWT_SECRET_LEN {
            warn!("⚠️  JWT_SECRET is shorter than recommended (32+ characters)");
        }

        info!("✅ Configuration validation passed");
        Ok(())
    }

    pub fn display_startup_info(&self) {
        info!("🚀 Starting JOS (Join Our Session) API");
        info!(
            "📊 Environment: {}",
            if cfg!(debug_assertions) {
                "Development"
            } else {
                "Production"
            }
        );
        info!("🌐 Server will bind to: {}", self.addr);
        info!(
            "🗄️  Database: {}",
            self.database_url.split('@').next_back().unwrap_or("unknown")
        );
        info!(
            "🔐 JWT expiration: {} days",
            self.jwt_expiration_duration.num_days()
        );
    }
}
