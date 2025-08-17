use chrono::Duration;

// only for development purposes, should not be used in production
pub const DEFAULT_JWT_SECRET: &str = "default_jwt_secret_1234567890";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_JWT_EXPIRATION_DURATION: Duration = Duration::days(1);
pub const MIN_JWT_SECRET_LEN: usize = 32;
