use crate::{domain::type_wraper::TypeWrapped, error::UserValidationError};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use unicode_segmentation::UnicodeSegmentation;

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DisplayName(String);

const MAX_DISPLAY_NAME_LENGTH: usize = 30;
const MIN_DISPLAY_NAME_LENGTH: usize = 5;

impl TypeWrapped for DisplayName {
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

    #[test]
    fn should_fail_if_display_name_is_too_long() {
        let too_long = "a".repeat(31);
        assert!(
            DisplayName::parse(too_long).is_err(),
            "Expected failure for name longer than max length"
        );
    }

    #[test]
    fn should_fail_if_display_name_is_empty_or_whitespace() {
        let empty_cases = vec!["", "   ", "\n", "\t", "\u{200B}"];
        empty_cases.iter().for_each(|case| {
            assert!(
                DisplayName::parse(case.to_string()).is_err(),
                "Expected failure for empty or whitespace input: {:?}",
                case
            );
        });
    }

    #[test]
    fn should_accept_unicode_composed_characters() {
        let composed = "Joãozinho 🦀";
        assert!(
            DisplayName::parse(composed.to_string()).is_ok(),
            "Expected valid composed unicode string"
        );
    }

    #[test]
    fn raw_should_return_original_trimmed_string() {
        let name = "   Alice 🚀   ";
        let parsed = DisplayName::parse(name.to_string()).unwrap();
        assert_eq!(parsed.raw(), "Alice 🚀".to_string());
    }

    #[test]
    fn should_handle_tabs_and_newlines_correctly() {
        let weird_whitespace = "\n\t  Alice\n\t";
        let result = DisplayName::parse(weird_whitespace.to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().raw(), "Alice");
    }

    #[test]
    fn should_fail_if_only_invisible_characters() {
        let invisible = "\u{200B}\u{200C}\u{200D}";
        assert!(
            DisplayName::parse(invisible.to_string()).is_err(),
            "Expected failure for invisible characters"
        );
    }

    #[test]
    fn should_accept_display_names_with_exactly_min_and_max_length() {
        let min = "ABCDE"; // 5 graphemes
        let max = "A".repeat(30); // 30 graphemes
        assert!(
            DisplayName::parse(min.to_string()).is_ok(),
            "Min length check failed"
        );
        assert!(
            DisplayName::parse(max.to_string()).is_ok(),
            "Max length check failed"
        );
    }

    #[test]
    fn should_handle_emoji_as_single_grapheme_properly() {
        let five_emojis = "🚀🚀🚀🚀🚀"; // 5 graphemes
        let thirty_emojis = "🚀".repeat(30); // 30 graphemes

        assert!(
            DisplayName::parse(five_emojis.into()).is_ok(),
            "Emoji min length check failed"
        );
        assert!(
            DisplayName::parse(thirty_emojis.clone()).is_ok(),
            "Emoji max length check failed"
        );

        let too_many_emojis = "🚀".repeat(31);
        assert!(
            DisplayName::parse(too_many_emojis).is_err(),
            "Should fail with 31 graphemes"
        );
    }
}
