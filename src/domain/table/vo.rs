use super::error::DescriptionValidationError;
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TitleVo(String);

const MIN_DESCRIPTION_LENGTH: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 1000;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DescriptionVo(String);

impl TypeWrapped for DescriptionVo {
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

        Ok(DescriptionVo(trimmed.to_string()))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}
