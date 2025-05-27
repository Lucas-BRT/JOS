use crate::domain::user::error::UserDomainError;
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UsernameVo(String);

impl TypeWrapped for UsernameVo {
    type Raw = String;
    type Error = UserDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let raw = raw.trim().to_string();

        Ok(Self(raw))
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

        valid_usernames.par_iter().for_each(|username| {
            assert!(
                UsernameVo::parse(username.to_string()).is_ok(),
                "Expected to pass: {}",
                username
            );
        });
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

        invalid_usernames.par_iter().for_each(|username| {
            assert!(
                UsernameVo::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        });
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

        invalid_usernames.par_iter().for_each(|username| {
            assert!(
                UsernameVo::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        });
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

        invalid_usernames.par_iter().for_each(|username| {
            assert!(
                UsernameVo::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        });
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

        invalid_usernames.par_iter().for_each(|username| {
            assert!(
                UsernameVo::parse(username.to_string()).is_err(),
                "Unexpectedly passed: {}",
                username
            );
        });
    }
}
