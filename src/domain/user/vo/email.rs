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
