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
mod tests {}
