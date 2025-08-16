use crate::core::setup::SetupError;
use crate::{Error, Result};

pub fn validate_environment() -> Result<()> {
    let required_vars = vec!["DATABASE_URL", "PORT", "JWT_SECRET"];

    let mut missing_vars = Vec::new();

    for var in required_vars {
        if std::env::var(var).is_err() {
            missing_vars.push(var);
        }
    }

    if !missing_vars.is_empty() {
        return Err(Error::Setup(SetupError::EnvironmentValidationFailed(
            format!(
                "Missing required environment variables: {}",
                missing_vars.join(", ")
            ),
        )));
    }

    Ok(())
}
