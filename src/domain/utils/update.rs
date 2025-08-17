use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub enum Update<T> {
    #[default]
    Keep,
    Change(T),
}
