use crate::domain::user::error::UserDomainError;
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DisplayNameVo(String);

impl TypeWrapped for DisplayNameVo {
    type Raw = String;
    type Error = UserDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        Ok(DisplayNameVo(raw))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}
