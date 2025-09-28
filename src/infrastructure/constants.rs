use chrono::Duration;

pub const DEFAULT_JWT_SECRET: &str = "default_jwt_secret_1234567890";
pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_JWT_EXPIRATION_DURATION: Duration = Duration::days(1);
pub const MIN_JWT_SECRET_LEN: usize = 32;
