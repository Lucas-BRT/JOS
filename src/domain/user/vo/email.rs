use crate::domain::{
    user::error::{EmailDomainError, UserDomainError},
    utils::type_wraper::TypeWrapped,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct EmailVo {
    #[validate(email)]
    mail: String,
}

impl TypeWrapped for EmailVo {
    type Raw = String;
    type Error = UserDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        if let Err(e) = (Self { mail: raw.clone() }).validate() {
            return Err(UserDomainError::Email(EmailDomainError::InvalidEmail(
                e.to_string(),
            )));
        }
        Ok(EmailVo { mail: raw })
    }

    fn raw(&self) -> Self::Raw {
        self.mail.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_accept_valid_emails() {
        let valid_emails = vec![
            "user@example.com",
            "john.doe@domain.co.uk",
            "alice+bob@sub.domain.io",
            "user123@something.net",
            "test.email+alex@leetcode.com",
        ];

        for email in valid_emails {
            assert!(
                EmailVo::parse(email.to_string()).is_ok(),
                "Expected valid email: {}",
                email
            );
        }
    }

    #[test]
    fn should_reject_invalid_emails() {
        let invalid_emails = vec![
            "",
            "plainaddress",
            "@no-local-part.com",
            "Outlook Contact <outlook-contact@domain.com>",
            "no-at.domain.com",
            "user@.com",
            // "user@com", awaiting until the validation crate updates
            "user@domain..com",
            "user@domain,com",
            // "user@domain",
        ];

        for email in invalid_emails {
            assert!(
                EmailVo::parse(email.to_string()).is_err(),
                "Expected invalid email to fail: {}",
                email
            );
        }
    }

    #[test]
    fn should_return_original_string_with_raw_method() {
        let original = "some.user@site.com";
        let parsed = EmailVo::parse(original.to_string()).unwrap();
        assert_eq!(parsed.raw(), original);
    }

    #[test]
    fn should_fail_with_proper_error_message() {
        let invalid = "not-an-email";
        let err = EmailVo::parse(invalid.to_string()).unwrap_err();

        assert!(matches!(
            err,
            UserDomainError::Email(EmailDomainError::InvalidEmail(_))
        ));
    }
}
