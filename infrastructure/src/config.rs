use crate::{constants::*, setup::environment::Environment};
use chrono::Duration;
use shared::{Error, error::InfrastructureError};
use std::{net::SocketAddr, str::FromStr};
use tracing::{info, warn};

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
            jwt_expiration_duration: Duration::days(1),
            environment: Environment::Development,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Error> {
        match dotenvy::dotenv() {
            Ok(_) => info!("✅ Environment variables loaded from .env file"),
            Err(_) => warn!("⚠️  No .env file found, using system environment variables"),
        }

        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            Error::Infrastructure(InfrastructureError::FailedToGetEnvironmentVariable(
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

        let addr =
            SocketAddr::from_str(&format!("{DEFAULT_HOST}:{server_port}")).map_err(|err| {
                Error::Infrastructure(InfrastructureError::FailedToSetupServerAddress(
                    err.to_string(),
                ))
            })?;

        let environment = std::env::var("ENVIRONMENT")
            .map_err(|e| InfrastructureError::FailedToGetEnvironmentVariable(e.to_string()))
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
                    return Err(Error::Infrastructure(
                        InfrastructureError::FailedToGetEnvironmentVariable("JWT_SECRET".into()),
                    ));
                }
            },
            Environment::Development => jwt_secret_env.unwrap_or_else(|_| {
                warn!("⚠ Failed to read JWT_SECRET. Using default");
                DEFAULT_JWT_SECRET.into()
            }),
        };

        let mut jwt_expiration_duration = std::env::var("JWT_EXPIRATION_DURATION")
            .ok()
            .and_then(|value| value.parse::<i64>().ok())
            .map(Duration::days)
            .unwrap_or(DEFAULT_JWT_EXPIRATION_DURATION);

        if jwt_expiration_duration.num_days() < 1 {
            warn!("⚠️  JWT_EXPIRATION_DURATION is less than 1 day, setting to 1 day");
            jwt_expiration_duration = Duration::days(1);
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
            self.jwt_expiration_duration.num_days()
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
        info!(
            "🔐 JWT expiration: {} days",
            self.jwt_expiration_duration.num_days()
        );
    }
}
