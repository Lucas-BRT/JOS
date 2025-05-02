use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::UserValidationError;

const MAX_USERNAME_LENGTH: usize = 30;
const MIN_USERNAME_LENGTH: usize = 5;

#[derive(FromRow, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn parse(name: String) -> Result<Self, UserValidationError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(UserValidationError::Username(
                "username cannot be empty".to_string(),
            ));
        }

        if name.len() > MAX_USERNAME_LENGTH {
            return Err(UserValidationError::Username(format!(
                "username cannot have more than {MAX_USERNAME_LENGTH} characters"
            )));
        }

        if name.len() < MIN_USERNAME_LENGTH {
            return Err(UserValidationError::Username(format!(
                "username cannot have less than {MIN_USERNAME_LENGTH} characters"
            )));
        }

        if let Some(first_char) = name.chars().next() {
            if first_char == '_' {
                return Err(UserValidationError::Username(
                    "username cannot begin with underscore".to_string(),
                ));
            }
        }

        if let Some(last_char) = name.chars().last() {
            if last_char == '_' {
                return Err(UserValidationError::Username(
                    "username cannot end with underscore".to_string(),
                ));
            }
        }

        if name.chars().any(|c| !(c.is_ascii_lowercase() || c == '_')) {
            return Err(UserValidationError::Username(
                "username can only contain lowercase ASCII letters and underscores".to_string(),
            ));
        }

        Ok(Self(name.into()))
    }
}

impl ToString for Username {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_allow_valid_usernames() {
        let valid_usernames = [
            "bighero",
            "big_hero",
            "a_b_c_d",
            "abcdefghijabcdefghijabcde", // 30 chars
            "valid_user_name",
            "user_name_test",
            "usernameusernameusernam", // 25 chars
            "validname",
            "hero_name",
            "dark_goblin",
        ];

        for username in valid_usernames {
            assert!(
                Username::parse(username.to_string()).is_ok(),
                "Expected to pass: {}",
                username
            );
        }
    }

    #[test]
    fn should_fail_if_starts_with_underscore() {
        let invalid_usernames = [
            "_bighero",
            "_big_hero",
            "_a_b_c",
            "_abcdefghijabcdefghijabcde",
            "_valid_user_name",
            "_user_name_test",
            "_usernameusernameusernam",
            "_validname",
            "_hero_name",
            "_dark_goblin",
        ];

        for username in invalid_usernames {
            assert!(
                Username::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        }
    }

    #[test]
    fn should_fail_if_ends_with_underscore() {
        let invalid_usernames = [
            "bighero_",
            "big_hero_",
            "hero_",
            "a_b_c_",
            "test_user_",
            "dark_goblin_",
            "username_",
            "example_",
            "test123_",
            "longusernamewithunderscore_",
        ];

        for username in invalid_usernames {
            assert!(
                Username::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        }
    }

    #[test]
    fn should_fail_if_too_short_or_too_long() {
        let invalid_usernames = [
            "a",                                 // too short
            "ab",                                // too short
            "abcd",                              // too short
            "abc",                               // too short
            "usernameusernameusernameusername1", // 35 chars
            "thisusernameiswaytoolongtobevalid", // > 30
            "",                                  // empty
            "    ",                              // spaces only
            "\n",                                // newline
            "\t\t",                              // tabs
        ];

        for username in invalid_usernames {
            assert!(
                Username::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {:?}",
                username
            );
        }
    }

    #[test]
    fn should_fail_if_contains_invalid_characters() {
        let invalid_usernames = [
            "bigher!o3",
            "abcdef39ghijefghijabcde", // contains digits
            "valid_us⚠️er_name",       // emoji
            "user name test",          // spaces
            "USERNAME",                // uppercase
            "user.name",               // dot
            "user-name",               // hyphen
            "user@name",               // at sign
            "héros",                   // accented
            "user$name",               // dollar sign
        ];

        for username in invalid_usernames {
            assert!(
                Username::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        }
    }
}
