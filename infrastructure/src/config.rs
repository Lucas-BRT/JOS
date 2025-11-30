use crate::constants::*;
use crate::setup::environment::Environment;
use shared::Result;
use shared::error::Error;
use shared::error::SetupError;
use std::time::Duration;
use std::{net::SocketAddr, str::FromStr};
use tracing::{info, warn};

pub const DEFAULT_JWT_EXPIRATION_DURATION: Duration = Duration::from_hours(24);

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub addr: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_duration: Duration,
    pub environment: Environment,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from_str("127.0.0.1:8080").unwrap(),
            database_url: "".to_string(),
            jwt_secret: "secret".to_string(),
            jwt_expiration_duration: DEFAULT_JWT_EXPIRATION_DURATION,
            environment: Environment::Development,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        match dotenvy::dotenv() {
            Ok(_) => info!("✅ Environment variables loaded from .env file"),
            Err(_) => warn!("⚠️  No .env file found, using system environment variables"),
        }

        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            Error::Setup(SetupError::FailedToGetEnvironmentVariable(
                "DATABASE_URL".into(),
            ))
        })?;

        let server_port: u32 = match std::env::var("PORT") {
            Ok(port) => port.parse().unwrap_or_else(|_| {
                warn!(
                    "⚠ Failed to read server port. Using default port: {}",
                    DEFAULT_PORT
                );
                DEFAULT_PORT
            }),
            Err(_) => {
                warn!(
                    "⚠ Failed to read server port. Using default port: {}",
                    DEFAULT_PORT
                );
                DEFAULT_PORT
            }
        };

        let addr = SocketAddr::from_str(&format!("{DEFAULT_HOST}:{server_port}"))
            .map_err(|err| Error::Setup(SetupError::FailedToSetupServerAddress(err.to_string())))?;

        let environment = std::env::var("ENVIRONMENT")
            .map_err(|e| SetupError::FailedToGetEnvironmentVariable(e.to_string()))
            .map(|value| match value.to_ascii_lowercase().as_str() {
                "production" => Environment::Production,
                _ => Environment::Development,
            })
            .unwrap_or_default();

        info!("⚠ Running as {:?}", environment);

        let jwt_secret_env = std::env::var("JWT_SECRET");

        let jwt_secret = match environment {
            Environment::Production => match jwt_secret_env {
                Ok(secret) => secret,
                Err(_) => {
                    return Err(Error::Setup(SetupError::FailedToGetEnvironmentVariable(
                        "JWT_SECRET".into(),
                    )));
                }
            },
            Environment::Development => jwt_secret_env.unwrap_or_else(|_| {
                warn!("⚠ Failed to read JWT_SECRET. Using default");
                DEFAULT_JWT_SECRET.into()
            }),
        };

        let mut jwt_expiration_duration = std::env::var("JWT_EXPIRATION_DURATION")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_hours)
            .unwrap_or(DEFAULT_JWT_EXPIRATION_DURATION);

        let one_day_in_secs = Duration::from_hours(24).as_secs();

        if jwt_expiration_duration.as_secs() < one_day_in_secs {
            warn!("⚠️  JWT_EXPIRATION_DURATION is less than 1 day, setting to 1 day");
            jwt_expiration_duration = DEFAULT_JWT_EXPIRATION_DURATION;
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
            self.database_url
                .split('@')
                .next_back()
                .unwrap_or("unknown"),
            (self.jwt_expiration_duration.as_secs() / (60 * 60 * 24))
        )
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
            self.database_url
                .split('@')
                .next_back()
                .unwrap_or("unknown")
        );
    }
}
