use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Update<T> {
    Change(T),
    #[default]
    Keep,
}

impl<T> Update<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            Update::Change(value) => Some(value),
            Update::Keep => None,
        }
    }
}

impl<T> From<Option<T>> for Update<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(value) => Update::Change(value),
            None => Update::Keep,
        }
    }
}
