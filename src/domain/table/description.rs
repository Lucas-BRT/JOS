use serde::{Deserialize, Serialize};

use crate::{domain::type_wraper::TypeWrapped, error::DescriptionValidationError};

const MIN_DESCRIPTION_LENGTH: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 1000;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Description(String);

impl TypeWrapped for Description {
    type Raw = String;
    type Error = DescriptionValidationError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed = raw.trim();

        if trimmed.len() < MIN_DESCRIPTION_LENGTH {
            return Err(DescriptionValidationError::TooShort);
        }

        if trimmed.len() > MAX_DESCRIPTION_LENGTH {
            return Err(DescriptionValidationError::TooLong);
        }

        Ok(Description(trimmed.to_string()))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}
