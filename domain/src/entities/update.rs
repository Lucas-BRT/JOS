use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Update<T> {
    #[default]
    Keep,
    Change(T),
}

impl<T> From<Option<T>> for Update<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Update::Change(value),
            None => Update::Keep,
        }
    }
}
