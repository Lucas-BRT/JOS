use super::error::TableDomainError;
use crate::domain::utils::type_wraper::TypeWrapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TitleVo(String);

impl TypeWrapped for TitleVo {
    type Raw = String;
    type Error = TableDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed = raw.trim();


        Ok(TitleVo(trimmed.to_string()))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DescriptionVo(String);

impl TypeWrapped for DescriptionVo {
    type Raw = String;
    type Error = TableDomainError;

    fn parse(raw: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed = raw.trim();

        Ok(DescriptionVo(trimmed.to_string()))
    }

    fn raw(&self) -> Self::Raw {
        self.0.clone()
    }
}
