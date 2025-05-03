use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use unicode_segmentation::UnicodeSegmentation;

use crate::{domain::validation::Validated, error::UserValidationError};

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DisplayName(String);

const MAX_DISPLAY_NAME_LENGTH: usize = 30;
const MIN_DISPLAY_NAME_LENGTH: usize = 5;

impl Validated for DisplayName {
    type Raw = String;
    type Error = UserValidationError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let raw = raw.trim().to_string();
        let graphemes = raw.graphemes(true).collect::<Vec<&str>>().len();

        if graphemes == 0 {
            return Err(UserValidationError::DisplayName(
                "displayname cannot be empty".to_string(),
            ));
        }

        if graphemes > MAX_DISPLAY_NAME_LENGTH || graphemes < MIN_DISPLAY_NAME_LENGTH {
            return Err(UserValidationError::DisplayName(format!(
                "displayname must have between {} and {} characters",
                MIN_DISPLAY_NAME_LENGTH, MAX_DISPLAY_NAME_LENGTH
            )));
        }

        Ok(DisplayName(raw))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

    #[test]
    fn should_allow_valid_display_names() {
        let valid_display_names = vec![
            "John Doe",
            "Jane Smith",
            "Alice Johnson",
            "Bob Brown",
            "Charlie Davis",
            "David Miller",
            "Emily Wilson",
            "Frank Anderson",
            "Grace Thomas",
            "Henry Jackson",
            "__NobodyMike__",
            "Sky 🚀 Emily",
        ];

        valid_display_names.par_iter().for_each(|display_name| {
            assert!(
                DisplayName::parse(display_name.to_string()).is_ok(),
                "Testing display name: {}",
                display_name
            );
        });
    }

    #[test]
    fn should_fail_if_display_name_is_too_short() {
        let invalid_display_names = vec![
            "Jo",
            "Ja",
            "Al",
            "Bo",
            "Ch",
            "Da",
            "Em",
            "Fr",
            "Gr",
            "He",
            "__No",
            "Ska🚀",
            "            Ska🚀", // should ignore whitespace
            "                       Ska",
        ];

        invalid_display_names.par_iter().for_each(|display_name| {
            assert!(
                DisplayName::parse(display_name.to_string()).is_err(),
                "Testing display name: {}",
                display_name
            );
        });
    }
}
